extern crate the_blue_alliance;
extern crate core;
extern crate simplelog;
extern crate log;
extern crate rusty_machine;
extern crate indicatif;
use core::cmp::Ordering;
use the_blue_alliance::TBA;
use the_blue_alliance::team::Team;
use the_blue_alliance::matches::Match;
use rusty_machine::learning::nnet::BCECriterion;
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::learning::nnet::NeuralNet;
use rusty_machine::learning::toolkit::regularization::Regularization;
use std::sync::Mutex;
use std::borrow::BorrowMut;
use std::thread;
use std::io;
use std::io::Write;

const LAYERS: [usize; 5] = [3,5,11,7,3];

fn main() {

    simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default()).expect("Failed to init logger.");
    let mut tba = Mutex::new(TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH"));

    let criterion = BCECriterion::new(Regularization::L2(0.1));
    let mut nnet = NeuralNet::new(&LAYERS, criterion, StochasticGD::default());

    //TODO: Make this not deadlock.
    let test: Vec<the_blue_alliance::matches::Match> = the_blue_alliance::district::District::in_year(tba.try_lock().expect("Cannot lock TBA struct.").borrow_mut(), 2018).expect("Cannot get districts for year 2018").into_iter().enumerate().flat_map( |(i, district)| {
        print!("d");
        io::stdout().flush();
        district.events(tba.try_lock().expect("Cannot lock TBA struct.").borrow_mut()).expect("Failed to get events for district.").into_iter()
    }).enumerate().flat_map(|(i, events)| {
        print!("e");
        io::stdout().flush();
        events.matches(tba.try_lock().expect("Cannot lock TBA struct.").borrow_mut()).expect("Failed to get matches for event.").into_iter()
    }).collect();


}
