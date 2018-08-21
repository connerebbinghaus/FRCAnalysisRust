use std::collections::HashMap;
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
    pub is_opr_reliable: bool,
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
            is_opr_reliable: false,
        }
    }

    fn update(&mut self, data: &Alliances)  {

        let red_find = data.red.team_keys.contains(&self.team_key);
        let blue_find = data.blue.team_keys.contains(&self.team_key);

        let alliance = match (red_find, blue_find) {
            (true, false) => Some(Alliance::Red),
            (false, true) => Some(Alliance::Blue),
            (false, false) => None,
            (true, true) => panic!("Team is on both alliances!"),
        };

        if let Some(team) = alliance {
            self.played += 1;

            match (team, data.red.score.cmp(&data.blue.score)) {
                (Alliance::Red, Ordering::Greater) => {
                    self.wins += 1
                },
                (Alliance::Red, Ordering::Less) => {
                    self.losses += 1
                },
                (Alliance::Red, Ordering::Equal) => {
                    self.ties += 1
                },
                (Alliance::Blue, Ordering::Greater) => {
                    self.losses += 1
                },
                (Alliance::Blue, Ordering::Less) => {
                    self.wins += 1
                },
                (Alliance::Blue, Ordering::Equal) => {
                    self.ties += 1
                },
            }
        }
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

            self.states.extend(prev.states.keys()
                .map(|key: &String| {
                    let mut state = prev.states.get(key.as_str())
                        .expect("Previous match does not have data it should.").clone();
                    state.update(&alliances);
                    (key.clone(), state)
                }));

            self.states.extend(alliances.blue.team_keys.iter()
                .map(|key: &String| {
                    let new_state = TeamState::new(&key);
                    let mut state = prev.states.get(key.as_str())
                        .or_else(|| Some(&new_state))
                        .unwrap().clone();
                    state.update(&alliances);
                    (key.clone(), state)
                }));
            self.states.extend(alliances.red.team_keys.iter()
                .map(|key: &String| {
                    let new_state = TeamState::new(&key);
                    let mut state = prev.states.get(key.as_str())
                        .or_else(|| Some(&new_state))
                        .unwrap().clone();
                    state.update(&alliances);
                    (key.clone(), state)
                }));
        }
    }

    fn simulate_first(&mut self) {
        if let Some(ref alliances) = self.inner.alliances {
            self.states.extend(alliances.blue.team_keys.iter()
                .map(|key: &String| {
                    let mut state = TeamState::new(&key);
                    state.update(&alliances);
                    (key.clone(), state)
                }));
            self.states.extend(alliances.red.team_keys.iter()
                .map(|key: &String| {
                    let mut state = TeamState::new(&key);
                    state.update(&alliances);
                    (key.clone(), state)
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
                let is_reliable = oprs.is_reliable();
                for (team, opr) in oprs.unwrap() {
                    a_match.states.entry(team.clone()).and_modify(|state| {
                        state.is_opr_reliable = is_reliable;
                        state.opr = opr;
                    }).or_insert_with(|| {
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
        trace!("Simulating event...");
        matches.sort();
        let matches: Vec<_> = matches.into_iter()
            .map(|m| Arc::new(Mutex::new(SimulatedMatch::new(m))))
            .collect();

        if matches.len() > 2 {
            matches.first().unwrap().lock().unwrap().simulate_first();

            for m in matches.windows(2) {
                let mut a_match = m[1].lock().unwrap();
                let prev = m[0].lock().unwrap();
                a_match.simulate(&*prev);
            }
        }

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

