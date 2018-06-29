use the_blue_alliance::matches::Match;
use std::collections::HashMap;
use the_blue_alliance::matches::Winner;

pub mod opr;
//
//pub struct TeamState {
//    team_key: String,
//    played: usize,
//    wins: usize,
//    losses: usize,
//    ties: usize
//}
//
//impl TeamState {
//    fn win(&self) -> TeamState {
//        TeamState {
//            team_key: self.team_key.clone(),
//            played: self.played + 1,
//            wins: self.wins + 1,
//            losses: self.losses,
//            ties: self.ties,
//        }
//    }
//
//    fn lose(&self) -> TeamState {
//        TeamState {
//            team_key: self.team_key.clone(),
//            played: self.played + 1,
//            wins: self.wins,
//            losses: self.losses + 1,
//            ties: self.ties,
//        }
//    }
//
//    fn tie(&self) -> TeamState {
//        TeamState {
//            team_key: self.team_key.clone(),
//            played: self.played + 1,
//            wins: self.wins,
//            losses: self.losses,
//            ties: self.ties + 1,
//        }
//    }
//}
//
//pub struct Simulation {
//    matches: HashMap<String, Vec<Match>>
//}
//
//impl Simulation {
//    pub fn new() -> Simulation {
//        Simulation {
//            matches: HashMap::new(),
//        }
//    }
//
//    pub fn add_match(&mut self, m: Match) {
//        let matches_for_event = self.matches.get_mut(m.event_key.as_str())
//            .or_else(|| {
//                self.matches.insert(m.event_key.clone(),  Vec::new());
//                self.matches.get_mut(m.event_key.as_str())
//            }).unwrap();
//        matches_for_event.push(m);
//    }
//
//    fn simulate_single(prev_state: &TeamState, the_match: &Match) -> Option<TeamState> {
//        enum Alliance {
//            Red,
//            Blue
//        }
//
//        let team = &prev_state.team_key;
//        match the_match.alliances {
//            None => None,
//            Some(ref alliances) => {
//                let alliance = match alliances.red.team_keys.contains(team) {
//                    true => Some(Alliance::Red),
//                    false => match alliances.blue.team_keys.contains(team) {
//                        true => Some(Alliance::Blue),
//                        false => None,
//                    },
//                };
//
//                match alliance {
//                    None => None,
//                    Some(Alliance::Red) => {
//                        match the_match.winning_alliance {
//                            None => None,
//                            Some(Winner::Red) => Some(prev_state.win()),
//                            Some(Winner::Blue) => Some(prev_state.lose()),
//                            Some(Winner::None) => Some(prev_state.tie())
//                        }
//                    },
//                    Some(Alliance::Blue) => {
//                        match the_match.winning_alliance {
//                            None => None,
//                            Some(Winner::Blue) => Some(prev_state.win()),
//                            Some(Winner::Red) => Some(prev_state.lose()),
//                            Some(Winner::None) => Some(prev_state.tie())
//                        }
//                    },
//                }
//            },
//        }
//    }
//
//    fn simulate_match(&mut self) {
//
//    }
//}