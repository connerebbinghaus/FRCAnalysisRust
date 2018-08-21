use simulation::SimulatedEvent;
use simulation::SimulatedMatch;
use std::collections::HashMap;
use itertools::Itertools;
use itertools::flatten;
use std::cmp::Ordering;
use the_blue_alliance::matches::Winner;
use std::ops::Range;
use num_traits::real::Real;
use fann;
use std::sync::Mutex;
use std::mem;

static LAYERS: [u32; 2] = [18,2];
static SCORE_RANGE: Range<f32> = 0.0..400.0;
static NNET_RANGE: Range<f32> = -1.0..1.0;
static TRAIN_SUBSET_AMOUNT: f32 = 0.5;
static MAX_NEURONS: u32 = 50;

fn map_value<T: Real>(val: T, from: &Range<T>, to: &Range<T>) -> T {
    let val = val.min(from.end).max(from.start);

    (val-from.start)/(from.end-from.start) * (to.end-to.start) + to.start
}

pub struct Predictor {
    neural_net: fann::Fann,
}

impl Predictor {
    pub fn new() -> Predictor {
        let mut nnet = fann::Fann::new_shortcut(&LAYERS).expect("Failed to create neural network.");
        nnet.set_train_algorithm(fann::TrainAlgorithm::Rprop(Default::default()));
        nnet.set_stop_func(fann::StopFunc::Mse);
        Predictor {
            neural_net: nnet
        }
    }

    pub fn train(&mut self, events: &Vec<SimulatedEvent>, world_oprs: &HashMap<String, f32>) {
        info!("Training neural network...");
        debug!("Creating inputs...");
        let mut skipped_matches = Vec::new();

        let (inputs, outputs): (Vec<_>, Vec<_>) = flatten(events.iter()
            .map(|event| event.matches.iter()))
            .tuple_windows::<(_, _)>()
            .map(|(prev_match, a_match): (&SimulatedMatch, &SimulatedMatch)| {
                if let Some(ref alliances) = a_match.inner.alliances {
                    (flatten(a_match.inner.team_keys().unwrap().iter()
                        .map(|team| prev_match.states.get(team.as_str()))
                        .map(|state| {
                            if let Some(state) = state {
                                vec![
                                    (state.wins as f32 / state.played as f32).max(0.0f32),
                                    if state.is_opr_reliable {
                                        map_value(state.opr as f32, &SCORE_RANGE, &NNET_RANGE)
                                    } else {
                                        -1f32
                                    },
                                    *world_oprs.get(state.team_key.as_str()).or(Some(&-1f32)).unwrap()
                                ].into_iter()
                            } else {
                                skipped_matches.push(a_match.inner.key.clone());
                                Vec::new().into_iter()
                            }
                        }))
                         .collect_vec(),
                     vec![
                         map_value(alliances.red.score as f32, &SCORE_RANGE, &NNET_RANGE),
                         map_value(alliances.blue.score as f32, &SCORE_RANGE, &NNET_RANGE)
                     ]
                    )
                } else {
                    (Vec::new(), Vec::new())
                }
            })
            .filter(|v| v.0.len() == LAYERS[0] as usize && v.1.len() == LAYERS[1] as usize)
            .unzip();

        debug!("Training...");
        let len = inputs.len() as u32;

        let inputs = Mutex::new(inputs.into_iter());
        let outputs= Mutex::new(outputs.into_iter());

        let mut data = fann::TrainData::from_callback(len, LAYERS[0], LAYERS[1],
                                                      Box::new(move |_i| (inputs.lock().unwrap().next().unwrap(), outputs.lock().unwrap().next().unwrap())))
            .expect("Failed to create training data");

        data.shuffle();

        let num_to_take = (data.length() as f32 * TRAIN_SUBSET_AMOUNT) as u32;

        let data = data.subset(0, num_to_take)
            .expect("Failed to subset training data.");

        let cascade_params = fann::CascadeParams {
            .. fann::CascadeParams::default()
        };

        self.neural_net.set_cascade_params(&cascade_params);
        {
            let max_steps = MAX_NEURONS - self.neural_net.get_total_neurons();

            let train = self.neural_net.on_data(&data);

            let best_error = Mutex::new(f32::max_value());

            let cb = |fann: &fann::Fann, _data: &fann::TrainData, intervals: u32| {
                let error = fann.get_mse();

                let mut b_error = best_error.lock().unwrap();
                if error.lt(&b_error) {
                    info!("Training status: Iterations: {}, Neurons: {}, Error: {} (best)", intervals, fann.get_total_neurons(), error);
                    *b_error = error;
                    fann.save("nnet.dat").expect("Cannot save nnet");
                } else {
                    info!("Training status: Iterations: {}, Neurons: {}, Error: {}", intervals, fann.get_total_neurons(), error);
                }
                fann::CallbackResult::Continue
            };

            let mut train = train.with_callback(1, &cb).cascade();
            train.train(max_steps, 0.05)
                .expect("Failed to train");
        }
        let best_nnet = fann::Fann::from_file("nnet.dat")
            .expect("Cannot load nnet");
        info!("Training finished. Best: {} neurons.", best_nnet.get_total_neurons());
        let _old_nnet = mem::replace(&mut self.neural_net, best_nnet);
    }

    pub fn predict(&self, mut the_match: SimulatedMatch, world_oprs: &HashMap<String, f32>) -> Option<SimulatedMatch> {
        let inputs = flatten(the_match.inner.team_keys().unwrap().iter()
            .map(|team| the_match.states.get(team.as_str()))
            .map(|state| {
                if let Some(state) = state {
                    vec![
                        (state.wins as f32 / state.played as f32).max(0.0f32),
                        if state.is_opr_reliable {
                            map_value(state.opr as f32, &SCORE_RANGE, &NNET_RANGE)
                        } else {
                            -1f32
                        },
                        *world_oprs.get(state.team_key.as_str()).or(Some(&-1f32)).unwrap()
                    ]
                } else{
                    Vec::new()
                }.into_iter()
            }))
            .collect_vec();

        if inputs.len() != LAYERS[0] as usize {
            return None;
        }

        assert!(inputs.iter().sum::<f32>().is_finite(), "NaNs.");

        let output = self.neural_net.run(&inputs)
            .expect("Failed to predict");


        debug!("Prediction results: {}, {}", output.get(0).unwrap(), output.get(1).unwrap());


        let mut output = output;
        the_match.inner.winning_alliance = if let Some(ref mut alliances) = the_match.inner.alliances {
            alliances.blue.score = map_value(output.remove(1), &NNET_RANGE, &SCORE_RANGE) as i32;
            alliances.red.score = map_value(output.remove(0), &NNET_RANGE, &SCORE_RANGE) as i32;
            match alliances.red.score.cmp(&alliances.blue.score){
                Ordering::Less => Some(Winner::Blue),
                Ordering::Equal => None,
                Ordering::Greater => Some(Winner::Red),
            }
        } else {None};
        Some(the_match)
    }
}