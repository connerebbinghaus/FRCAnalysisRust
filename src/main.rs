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
//extern crate fann;
extern crate chrono;
extern crate clap;
extern crate prettytable;
extern crate tui;
extern crate crossterm;

use std::iter::FromIterator;
use std::ops::Deref;
use chrono::TimeZone;
use futures::Future;
use the_blue_alliance::TBA;
use clap::{
    App, Arg, SubCommand
};

use std::collections::HashSet;
use std::collections::HashMap;


mod state;
mod kiosk;
mod opr;

fn main() {
    // pretty_env_logger::init();

    let pool = futures_cpupool::Builder::new()
        .after_start(|| debug!("Worker started."))
        .create();

    let tba = TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH", pool.clone());

    let matches = App::new("FRCAnalysis")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Conner Ebbinghaus <connerebbinghaus@gmail.com>")
        .about("Analyses data from The Blue Alliance.")
        .arg(Arg::with_name("year")
            .short("y")
            .long("year")
            .value_name("YEAR")
            .help("Operate on data for this year.")
            .takes_value(true))
        .arg(Arg::with_name("time")
            .short("t")
            .long("time")
            .value_name("TIME")
            .help("Give data for this time in RFC 2822 format (default is now.)")
            .takes_value(true))
        .subcommand(SubCommand::with_name("team")
            .about("Does something with a team")
            .arg(Arg::with_name("TEAM")
                .help("The team to operate on.")
                .required(true)
                .index(1))
            .subcommand(SubCommand::with_name("info")
                .about("Displays basic team info")
            )
            .subcommand(SubCommand::with_name("status")
                .about("Displays how a team is doing")
                .arg(Arg::with_name("event")
                    .short("e")
                    .long("event")
                    .value_name("EVENT")
                    .help("Which event to display stats for. Defaults to latest event.")
                    .takes_value(true))

            )
        )
        .subcommand(SubCommand::with_name("event")
            .about("Does something with an event")
            .arg(Arg::with_name("EVENT")
                .help("The event to operate on.")
                .required(true)
                .index(1))
            .subcommand(SubCommand::with_name("info")
                .about("Displays basic event info")
            )
            .subcommand(SubCommand::with_name("compare")
                .about("Compares teams across events.")
                .arg(Arg::with_name("EVENT2")
                    .help("The event to compare with.")
                    .required(true)
                    .index(1))
            )
        )
        .subcommand(SubCommand::with_name("kiosk")
            .about("Displays event info prettily")
            .arg(Arg::with_name("EVENT")
                .help("The event to display.")
                .required(true)
                .index(1))
        )
        .get_matches();

    let time = if let Some(t) = matches.value_of("time") {
        chrono::DateTime::parse_from_rfc2822(&t).unwrap().with_timezone(&chrono::Utc)
    } else {
        chrono::Utc::now()
    };

    match matches.subcommand() {
        ("team", Some(sub_matches)) => {
            let team = sub_matches.value_of("TEAM").expect("No team specified.");

            match sub_matches.subcommand() {
                ("info", Some(sub_matches)) => {
                    let info = the_blue_alliance::team::Team::from_key(&tba, team).wait().unwrap();
                    println!("Team #{}", info.team_number);
                    println!("Name: {}", info.nickname.clone().unwrap_or_else(|| "N/A".to_owned()));
                    println!("Motto: {}", info.motto.clone().unwrap_or_else(|| "N/A".to_owned()));
                    println!("Location: {}, {}, {}", info.city.clone().unwrap_or_else(|| "N/A".to_owned()), info.state_prov.clone().unwrap_or_else(|| "N/A".to_owned()), info.country.clone().unwrap_or_else(|| "N/A".to_owned()));
                    println!("Rookie year: {}", info.rookie_year);
                    println!("Website: {}", info.website.clone().unwrap_or_else(|| "N/A".to_owned()));
                },
                ("status", Some(sub_matches)) => {
                    let infos = the_blue_alliance::event::Event::for_team_key(&tba, team).wait().unwrap();
                    let info = if let Some(event) = sub_matches.value_of("event") {
                        infos.into_iter().find(|e| e.key == event).expect("Cannot find event")
                    } else {
                        infos.into_iter().filter(|e| chrono::Utc.from_utc_date(&e.start_date) < chrono::Utc::today()).max_by_key(|e| e.start_date).expect("No events found for team")
                    };

                    println!("Event Name: {}", info.name);

                    let event_info = state::EventState::new(&info, &tba, time.clone());

                    println!("Team rank: {}", event_info.rank_of_team(&team).unwrap());

                    let team_data = event_info.team_data(&team).unwrap();

                    if let state::TeamRankingData::S2019(dat) = team_data {
                        println!("Panking points: {}", dat.ranking_points);
                        println!("Matches played: {}", dat.played);
                        println!("Ranking Score: {}", dat.ranking_score);
                        println!("Cargo Points: {}", dat.cargo_points);
                        println!("Panel Points: {}", dat.panel_points);
                        println!("Climb Points: {}", dat.climb_points);
                        println!("Sandstorm Points: {}", dat.sandstorm_points);
                    }

                },

                _ => panic!("Invalid or missing subcommand!"),
            }

        },
        ("event", Some(sub_matches)) => {
            let event = sub_matches.value_of("EVENT").expect("No event specified.");
            match sub_matches.subcommand() {
                ("info", Some(_)) => {
                    let info = the_blue_alliance::event::Event::from_key(&tba, event).wait().unwrap();

                    println!("Event Name: {}", info.name);
                    println!("District: {}", info.district.as_ref().map(|d| d.display_name.deref()).unwrap_or("N/A"));
                    println!("Week: {}", info.week.map(|i| i+1).map(|i| i.to_string()).unwrap_or_else(|| "N/A".to_owned()));
                    println!("Location: {}, {}, {}", info.city.clone().unwrap_or_else(|| "N/A".to_owned()), info.state_prov.clone().unwrap_or_else(|| "N/A".to_owned()), info.country.clone().unwrap_or_else(|| "N/A".to_owned()));
                    println!("Teams:");
                    let mut teams = info.teams(&tba).wait().unwrap();
                    teams.sort_unstable_by_key(|t| t.team_number);
                    for t in  teams {
                        println!("- {}: {}", t.team_number, t.nickname.as_ref().map(|n| n.deref()).unwrap_or("N/A"));
                    }
                },
                ("compare", Some(sub_matches)) => {
                    let info = the_blue_alliance::event::Event::from_key(&tba, event).wait().unwrap();
                    
                    let event2 = sub_matches.value_of("EVENT2").expect("No event specified.");
                    let info2 = the_blue_alliance::event::Event::from_key(&tba, event2).wait().unwrap();

                    println!("Common teams: ");
                    
                    let mut teams = info.teams(&tba).wait().unwrap();
                    let mut teams2 = info2.teams(&tba).wait().unwrap();

                    let set1: HashSet<i32> = HashSet::from_iter(teams.iter().map(|t| t.team_number));
                    let set2: HashSet<i32> = HashSet::from_iter(teams2.iter().map(|t| t.team_number));
                    
                    teams.append(&mut teams2);
                    teams.sort_unstable_by_key(|t| t.team_number);
                    teams.dedup_by_key(|t| t.team_number);


                    let mut team_numbers: Vec<i32> = set1.union(&set2).cloned().collect();

                    team_numbers.sort();

                    for team in team_numbers
                    {
                        println!("- {}", team);
                    }
                },
                _ => panic!("Invalid or missing subcommand!"), 
            }
        },
        ("kiosk", Some(sub_matches)) => {
            let event = sub_matches.value_of("EVENT").expect("No event specified.");
            ::kiosk::run(&event, &tba).unwrap();
        },
        _ => panic!("Invalid or missing subcommand!"),
    }
    
}
