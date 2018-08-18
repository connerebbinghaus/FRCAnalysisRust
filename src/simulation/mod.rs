use std::collections::HashMap;
use itertools::Itertools;
use the_blue_alliance::matches::Match;
use the_blue_alliance::team::Alliances;
use the_blue_alliance::event::Event;
use std::cmp::Ordering;
use futures::Stream;
use std::sync::Mutex;
use std::sync::Arc;

pub mod opr;

#[derive(Clone)]
pub struct TeamState {
    pub team_key: String,
    pub played: usize,
    pub wins: usize,
    pub losses: usize,
    pub ties: usize,
    pub opr: f32,
}

enum Alliance {
    Red, Blue
}

impl TeamState {
    fn new(key: &str) -> TeamState {
        TeamState {
            team_key: key.to_string(),
            played: 0,
            wins: 0,
            losses: 0,
            ties: 0,
            opr: 0.0,
        }
    }

    fn update(&self, data: &Alliances, team: Alliance) -> TeamState {
        let mut ret = self.clone();

        ret.played += 1;

        match (team, data.red.score.cmp(&data.blue.score)) {
            (Alliance::Red, Ordering::Greater) => {
                ret.wins += 1
            },
            (Alliance::Red, Ordering::Less) => {
                ret.losses += 1
            },
            (Alliance::Red, Ordering::Equal) => {
                ret.ties += 1
            },
            (Alliance::Blue, Ordering::Greater) => {
                ret.losses += 1
            },
            (Alliance::Blue, Ordering::Less) => {
                ret.wins += 1
            },
            (Alliance::Blue, Ordering::Equal) => {
                ret.ties += 1
            },

        }

        ret
    }
}

#[derive(Clone)]
pub struct SimulatedMatch {
    pub inner: Match,
    pub states: HashMap<String, TeamState>,
}

impl SimulatedMatch {
    fn new(m: Match) -> SimulatedMatch{
        SimulatedMatch{
            inner: m,
            states: HashMap::new(),
        }
    }

    fn simulate(&mut self, prev: &SimulatedMatch) {
        if let Some(ref alliances) = self.inner.alliances {
            self.states.extend(alliances.blue.team_keys.iter()
                .map(|key: &String| {
                    let new_state = TeamState::new(&key);
                    (key.clone(), prev.states.get(key.as_str())
                        .or_else(|| Some(&new_state))
                        .unwrap().update(&alliances, Alliance::Blue))
                }));
            self.states.extend(alliances.red.team_keys.iter()
                .map(|key: &String| {
                    let new_state = TeamState::new(&key);
                    (key.clone(), prev.states.get(key.as_str())
                        .or_else(|| Some(&new_state))
                        .unwrap().update(&alliances, Alliance::Red))
                }));
        }
    }
}

pub struct SimulatedEvent {
    pub inner: Event,
    pub matches: Vec<SimulatedMatch>
}

impl SimulatedEvent {
    fn calc_oprs(&mut self) {
        let mut prev_matches = Vec::new();
        for a_match in &mut self.matches {
            let oprs = opr::calc_oprs_for_matches(&prev_matches);
            if let Some(oprs) = oprs {
                for (team, opr) in oprs {
                    a_match.states.entry(team.clone()).and_modify(|state| state.opr = opr).or_insert_with(|| {
                        let mut new_state = TeamState::new(&team);
                        new_state.opr = opr;
                        new_state
                    });
                }
            }
            prev_matches.push(a_match.inner.clone())
        }
    }
}

pub fn simulate<E, F: Stream<Item = (Event, Vec<Match>), Error = E>>(future: F) -> impl Stream<Item = SimulatedEvent, Error = E> {
    future.map(|(event, mut matches)| {
        matches.sort();
        let matches: Vec<_> = matches.into_iter()
            .map(|m| Arc::new(Mutex::new(SimulatedMatch::new(m))))
            .tuple_windows::<(Arc<Mutex<SimulatedMatch>>,Arc<Mutex<SimulatedMatch>>)>()
            .collect();

        for (prev, a_match) in &matches{
            let mut a_match = a_match.lock().unwrap();
            let prev = prev.lock().unwrap();
            a_match.simulate(&*prev);
        }

        let matches: Vec<_> = matches.into_iter()
            .map(|(p, m)| {
                drop(p);
                m
            }) .collect();

        let matches: Vec<_> = matches.into_iter()
            .map(|m| Arc::try_unwrap(m).ok().expect("Cannot unwrap Arc.").into_inner().expect("Cannot unwrap mutex"))
            .collect();

        let mut ret = SimulatedEvent{
            inner: event,
            matches,
        };
        ret.calc_oprs();
        ret
    })
}


