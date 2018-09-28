
use num_traits::real::Real;
use simulation::SimulatedMatch;
use simulation::Simulator;
use the_blue_alliance::matches::Winner;
use the_blue_alliance::matches::Match;
use data::{Data, QuerySelect};
use prediction::Backend;
use std::ops::Range;
use fann::{Fann, TrainAlgorithm, StopFunc};
use itertools;
use itertools::Itertools;
use std::sync::Mutex;
use fann;
use std::mem;
use std::cmp::Ordering;

static LAYERS: [u32; 2] = [18,2];
static SCORE_RANGE: Range<f32> = 0.0..400.0;
static NNET_RANGE: Range<f32> = -1.0..1.0;
static TRAIN_SUBSET_AMOUNT: f32 = 0.5;
static MAX_NEURONS: u32 = 50;

fn map_value<T: Real>(val: T, from: &Range<T>, to: &Range<T>) -> T {
    let val = val.min(from.end).max(from.start);

    (val-from.start)/(from.end-from.start) * (to.end-to.start) + to.start
}

pub struct FannSimple {
    neural_net: Fann,
}

impl FannSimple {
    pub fn new() -> FannSimple {
        let mut nnet = Fann::new_shortcut(&LAYERS).expect("Failed to create neural network.");
        nnet.set_train_algorithm(TrainAlgorithm::Rprop(Default::default()));
        nnet.set_stop_func(StopFunc::Mse);
        FannSimple {
            neural_net: nnet
        }
    }
}

impl Backend for FannSimple {
    fn train(&mut self, data: &Data, sim: &Simulator) {
        info!("Training neural network...");
        debug!("Creating inputs...");
        let mut skipped_matches = Vec::new();

        let events = data.query()
            .find_event()
            .in_season(2018)
            .choose(QuerySelect::All)
            .go().multiple().unwrap()
            .into_iter()
            .map(|event| sim.simulate_event(data, &event))
            .collect_vec();

        let world_oprs = sim.get_world_oprs(data).reliable().unwrap();

        let (inputs, outputs): (Vec<_>, Vec<_>) = itertools::flatten(events.iter()
            .map(|event| event.matches.iter())
            .map(|matches|{
                let mut new_skipped_matches = skipped_matches.clone();
                let ret = matches.tuple_windows::<(_, _)>()
                .map(|(prev_match, a_match): (&SimulatedMatch, &SimulatedMatch)| {
                    if let Some(ref alliances) = a_match.inner.alliances {
                        (itertools::flatten(a_match.inner.team_keys().unwrap().iter()
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
                                    new_skipped_matches.push(a_match.inner.key.clone());
                                    Vec::new().into_iter()
                                }
                            }))
                            .collect_vec(),
                        vec![
                            map_value(alliances.red.score as f32, &SCORE_RANGE, &NNET_RANGE),
                            map_value(alliances.blue.score as f32, &SCORE_RANGE, &NNET_RANGE)
                        ]
                        )
                    }else {
                        (Vec::new(), Vec::new())
                    }
            })
            .filter(|v| v.0.len() == LAYERS[0] as usize && v.1.len() == LAYERS[1] as usize).collect_vec();
            skipped_matches.extend(new_skipped_matches.into_iter());
            ret.into_iter()
        })).unzip();

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

    fn predict(&self, data: &Data, sim: &Simulator, the_match: &Match) -> Option<Winner> {
        let prev_match = data.query()
            .find_match()
            .before_match(the_match)?
            .choose(QuerySelect::Newest)
            .go().single()?;
        let world_oprs = sim.get_world_oprs(&data).reliable()?;
        let sim_match = sim.simulate(data, &prev_match)?;
        let inputs = itertools::flatten(the_match.team_keys()?.iter()
            .map(|team| sim_match.states.get(team.as_str()))
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

        let mut output = self.neural_net.run(&inputs)
            .expect("Failed to predict");


        debug!("Prediction results: {}, {}", output[0], output[1]);

        let blue_score = map_value(output.remove(1), &NNET_RANGE, &SCORE_RANGE) as i32;
        let red_score = map_value(output.remove(0), &NNET_RANGE, &SCORE_RANGE) as i32;
        match red_score.cmp(&blue_score){
            Ordering::Less => Some(Winner::Blue),
            Ordering::Equal => None,
            Ordering::Greater => Some(Winner::Red),
        }
    }

}
