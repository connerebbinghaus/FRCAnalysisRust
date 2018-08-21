#![warn(unused_extern_crates)]

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

use the_blue_alliance::TBA;
use futures::{Future, Stream};
use simulation::simulate;
use itertools::flatten;
use std::io::Write;
use simulation::SimulatedMatch;

mod simulation;
mod prediction;

fn main() {
    pretty_env_logger::init();

    let pool = futures_cpupool::Builder::new()
        .after_start(|| debug!("Worker started."))
        .create();

    let tba = TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH", pool.clone());

    let districts_stream = futures::sync::mpsc::spawn(
        the_blue_alliance::district::District::in_year(&tba, 2018).into_stream()
            .map(|districts| futures::stream::iter_ok::<_, the_blue_alliance::Error>(districts.into_iter()))
            .flatten(),
        &pool, 3);

    let tba1 = tba.clone();

    let events_stream = futures::sync::mpsc::spawn(
        districts_stream
            .map(move |district| district.events(&tba1).into_stream())
            .flatten()
            .map(|events| futures::stream::iter_ok::<_, the_blue_alliance::Error>(events.into_iter()))
            .flatten()
        , &pool, 10);

    let tba2 = tba.clone();

    let event_simulation_stream = futures::sync::mpsc::spawn(
        simulate(events_stream
            .map(move |event| (event.clone(), event.matches(&tba2).wait().unwrap()))
            .map(|(event, mut matches)| {
                matches.sort();
                (event, matches)
            }))
        , &pool, 25);

    let tba3 = tba.clone();

    let districts_stream = futures::sync::mpsc::spawn(
        the_blue_alliance::district::District::in_year(&tba3, 2018).into_stream()
            .map(|districts| futures::stream::iter_ok::<_, the_blue_alliance::Error>(districts.into_iter()))
            .flatten(),
        &pool, 3);

    let tba4 = tba.clone();

    let events_stream = futures::sync::mpsc::spawn(
        districts_stream
            .map(move |district| district.events(&tba4).into_stream())
            .flatten()
            .map(|events| futures::stream::iter_ok::<_, the_blue_alliance::Error>(events.into_iter()))
            .flatten()
        , &pool, 10);

    let tba5 = tba.clone();

    let matches_stream = futures::sync::mpsc::spawn(
        events_stream
            .map(move |event| event.matches(&tba5).into_stream())
            .flatten()
            .map(|matches| futures::stream::iter_ok::<_, the_blue_alliance::Error>(matches.into_iter()))
            .flatten()
        , &pool, 25);

    let world_oprs_future = futures::sync::oneshot::spawn(
        matches_stream.collect()
            .map(|matches| {
                info!("Calculating world oprs...");
                simulation::opr::calc_oprs_for_matches(&matches).expect("Cannot calculate world oprs.")
            }),
        &pool
    );

    let world_oprs = world_oprs_future.wait().unwrap().unwrap();
    let simulated_events: Vec<simulation::SimulatedEvent> = event_simulation_stream.collect().wait().unwrap();

    let mut predictor = prediction::Predictor::new();
    predictor.train(&simulated_events, &world_oprs);

    info!("Testing predictor...");

    let mut tested = 0;
    let mut correct = 0;

    for a_match in flatten(simulated_events.iter().map(|event| event.matches.iter())) {
        if a_match.inner.alliances.is_some() {
            let a_match: &SimulatedMatch = a_match;
            let prediction = predictor.predict(a_match.clone(), &world_oprs);
            if let Some(prediction) = prediction {
                tested += 1;
                if a_match.inner.winning_alliance == prediction.inner.winning_alliance {
                    correct += 1;
                }
            }
        }
    }

    info!("Tested: {}, Correct: {}, Accuracy: {}%", tested, correct, (correct as f32 /tested as f32) * 100.0);

    loop {
        print!("Enter a match to predict: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.to_lowercase().trim() == "quit" {
            break;
        }

        let the_match = flatten(simulated_events.iter()
            .map(|event| event.matches.iter()))
            .find(|a_match: &&simulation::SimulatedMatch| a_match.inner.key == input.trim())
            .expect("Match not found");

        let match_pred = predictor.predict(the_match.clone(), &world_oprs).unwrap();

        println!("Red: {}, Blue: {}", match_pred.inner.alliances.as_ref().unwrap().red.score,
                 match_pred.inner.alliances.as_ref().unwrap().blue.score);
        println!("Actual: {}, {}", the_match.inner.alliances.as_ref().unwrap().red.score,
                 the_match.inner.alliances.as_ref().unwrap().blue.score);
    }
}
