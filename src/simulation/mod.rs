use std::collections::HashMap;
use itertools::Itertools;
use the_blue_alliance::matches::Match;
use the_blue_alliance::matches::Winner;
use the_blue_alliance::event::Event;
use std::cmp::Ordering;

pub mod opr;

pub struct TeamState {
    team_key: String,
    played: usize,
    wins: usize,
    losses: usize,
    ties: usize,
    opr: f32
}

impl TeamState {
    fn win(&self) -> TeamState {
        TeamState {
            team_key: self.team_key.clone(),
            played: self.played + 1,
            wins: self.wins + 1,
            losses: self.losses,
            ties: self.ties,
            opr: self.opr,
        }
    }

    fn lose(&self) -> TeamState {
        TeamState {
            team_key: self.team_key.clone(),
            played: self.played + 1,
            wins: self.wins,
            losses: self.losses + 1,
            ties: self.ties,
            opr: self.opr,
        }
    }

    fn tie(&self) -> TeamState {
        TeamState {
            team_key: self.team_key.clone(),
            played: self.played + 1,
            wins: self.wins,
            losses: self.losses,
            ties: self.ties + 1,
            opr: self.opr,
        }
    }
}

enum SimulationResult {
    Ok([TeamState; 6]),
    Failed,
    None
}

pub struct Simulation {
    matches: HashMap<String, Match>,
    team_states: HashMap<String, SimulationResult>
}

impl Simulation {
    pub fn new(matches: Vec<Match>) -> Simulation {
        let mut matches_map = HashMap::new();

        matches.into_iter().for_each(|m| {
            matches_map.insert(m.key.clone(), m);
        });

        Simulation {
            matches: matches_map,
            team_states: HashMap::new(),
        }
    }

    fn matches_before(&self, a_match: &Match) -> impl Iterator<Item = &Match> {
        self.matches.values().filter(|mat| mat < a_match).collect()
    }

    fn simulate_single(prev_state: &TeamState, the_match: &Match) -> Option<TeamState> {
        enum Alliance {
            Red,
            Blue
        }

        let team = &prev_state.team_key;
        match the_match.alliances {
            None => None,
            Some(ref alliances) => {
                let alliance = match alliances.red.team_keys.contains(team) {
                    true => Some(Alliance::Red),
                    false => match alliances.blue.team_keys.contains(team) {
                        true => Some(Alliance::Blue),
                        false => None,
                    },
                };

                match alliance {
                    None => None,
                    Some(Alliance::Red) => {
                        match the_match.winning_alliance {
                            None => None,
                            Some(Winner::Red) => Some(prev_state.win()),
                            Some(Winner::Blue) => Some(prev_state.lose()),
                            Some(Winner::None) => Some(prev_state.tie())
                        }
                    },
                    Some(Alliance::Blue) => {
                        match the_match.winning_alliance {
                            None => None,
                            Some(Winner::Blue) => Some(prev_state.win()),
                            Some(Winner::Red) => Some(prev_state.lose()),
                            Some(Winner::None) => Some(prev_state.tie())
                        }
                    },
                }
            },
        }
    }

    fn simulate_match(&mut self, key: &String) {
        let a_match = self.matches.get(key).unwrap();
        for m in self.matches_before(a_match) {
            match self.team_states.get(&m.key) {
                None => self.simulate_match(&m.key),
                Some(&SimulationResult::None) => self.simulate_match(&m.key),
                Some(&SimulationResult::Failed) => {return;},
                Some(&SimulationResult::Ok(_)) => (),
            }
        }
    }
}