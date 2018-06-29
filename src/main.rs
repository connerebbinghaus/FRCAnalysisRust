extern crate the_blue_alliance;
extern crate simplelog;
#[macro_use]
extern crate log;
extern crate rusty_machine;
extern crate indicatif;
extern crate futures;
extern crate tokio_core;
extern crate nalgebra;
extern crate itertools;

use the_blue_alliance::TBA;
use rusty_machine::learning::nnet::BCECriterion;
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::learning::nnet::NeuralNet;
use rusty_machine::learning::toolkit::regularization::Regularization;
use std::sync::Mutex;
use std::borrow::BorrowMut;
use std::io;
use std::io::Write;
use itertools::Itertools;

const LAYERS: [usize; 5] = [3,5,11,7,3];
use futures::{Future, Stream};
use rusty_machine::linalg::Matrix;
use the_blue_alliance::matches::CompLevel;

mod simulation;
use simulation::opr::calc_oprs_for_matches;
fn main() {
    let mut log_conf = simplelog::Config::default();
    log_conf.target = Some(log::Level::Error);
    simplelog::SimpleLogger::init(log::LevelFilter::Debug, log_conf).expect("Failed to init logger.");
    let tba = TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH");

    let criterion = BCECriterion::new(Regularization::L2(0.1));
    let mut nnet = NeuralNet::new(&LAYERS, criterion, StochasticGD::default());
    let mut event_loop = tokio_core::reactor::Core::new()
        .expect("Cannot setup tokio core");

    let data = event_loop.run(the_blue_alliance::district::District::in_year(&tba, 2018).into_stream()
        .map( |districts| {
            futures::stream::iter_ok(districts.into_iter())
        })
        .flatten()
        .map(|district| {
            print!("d");
            io::stdout().flush();
            district.events(&tba).into_stream()
        })
        .flatten()
        .map(|events| futures::stream::iter_ok::<_, the_blue_alliance::Error>(events.into_iter()))
        .flatten()
        .map(|event| {
            print!("e");
            io::stdout().flush();
            event.matches(&tba).into_stream()
        })
        .flatten()
        .map(|matches| (matches.first().unwrap().clone(), simulation::opr::calc_oprs_for_matches(matches)))
        .collect()).unwrap();



    println!();

    data.into_iter().for_each(|(a_match, oprs)| {
        println!("{}:", a_match.key);
        if let Some(oprs) = oprs {
            for (team, opr) in oprs {
                println!("{}: {}", team, opr);
            }
        }
        else {
            println!("N/A");
        }
    });
}
