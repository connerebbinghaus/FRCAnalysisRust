#![warn(unused_extern_crates)]

extern crate the_blue_alliance;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate rusty_machine;
extern crate futures;
extern crate futures_cpupool;
extern crate nalgebra;
extern crate itertools;

use the_blue_alliance::TBA;
use futures::{Future, Stream};
use simulation::simulate;

mod simulation;

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

    let matches_stream = futures::sync::mpsc::spawn(
        simulate(events_stream
            .map(move |event| (event.clone(), event.matches(&tba2).wait().unwrap()))
            .map(|(event, mut matches)| {
                matches.sort();
                (event, matches)
            }))
        , &pool, 25);



    let fut = matches_stream.collect();
    let res = fut.wait().unwrap();
    for event in res {
        for a_match in event.matches {
            println!("{}", a_match.inner.key);
            for team in a_match.states.values() {
                println!("\t {}: {}, {}, {}, {}", team.team_key, team.wins, team.losses, team.ties, team.opr);
            }
        }
    }
    println!("Done!")
}
