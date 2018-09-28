#![warn(unused_extern_crates)]
#![warn(clippy)]

extern crate the_blue_alliance;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate futures;
extern crate futures_cpupool;
extern crate nalgebra;
extern crate itertools;
extern crate num_traits;
extern crate fann;
extern crate chrono;

use the_blue_alliance::TBA;
use std::sync::{Arc, RwLock};

mod simulation;
mod prediction;
mod data;

fn main() {
    pretty_env_logger::init();

    let pool = futures_cpupool::Builder::new()
        .after_start(|| debug!("Worker started."))
        .create();

    let tba = TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH", pool.clone());
    let data = Arc::new(RwLock::new(data::Data::new(tba)));
    let sim = Arc::new(RwLock::new(simulation::Simulator::new()));
    let pred = Arc::new(RwLock::new(prediction::Predictor::new()));

    
}
