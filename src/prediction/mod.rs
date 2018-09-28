use simulation::Simulator;
use the_blue_alliance::matches::Match;
use data::Data;
use the_blue_alliance::matches::Winner;

mod fann;

pub trait Backend {
    fn train(&mut self, data: &Data, sim: &Simulator,);
    fn predict(&self, data: &Data, sim: &Simulator, the_match: &Match) -> Option<Winner>;
}

pub struct Predictor {
    nnets: Vec<Box<Backend>>
}

impl Predictor {
    pub fn new() -> Predictor {
        Predictor {
            nnets: vec![
                Box::new(fann::FannSimple::new())
            ]
        }
    }

    pub fn train(&mut self, data: &Data, sim: &Simulator) {
        info!("Training {} prediction models...", self.nnets.len());
        for nnet in &mut self.nnets {
            nnet.train(data, sim);
        }
    }

    pub fn predict(&self, data: &Data, sim: &Simulator, the_match: &Match) -> Option<Winner> {
        let mut iter = self.nnets.iter();
        let mut nnet = iter.next();
        loop {
            if let Some(nnet) = nnet {
                if let Some(res) = nnet.predict(data, sim, the_match) {
                    break Some(res);
                }
            } else {
                break None;
            }
            nnet = iter.next();
        }
    }
}