use nalgebra::*;
use the_blue_alliance::matches::Match;
use std::collections::HashMap;
use std::mem; 
use itertools;

#[derive(Debug)]
pub enum Error {
    NoMatches,
    Unsolvable
}

#[derive(Clone, Debug)]
pub enum OprsOwned {
    Reliable(HashMap<String, f32>),
    Unreliable(HashMap<String, f32>),
}

impl OprsOwned {
    pub fn is_reliable(&self) -> bool {
        match self {
            OprsOwned::Reliable(_) => true,
            OprsOwned::Unreliable(_) => false,
        }
    }

    pub fn reliable(self) -> Option<HashMap<String, f32>> {
        match self {
            OprsOwned::Reliable(d) => Some(d),
            OprsOwned::Unreliable(_) => None,
        }
    }

    pub fn unwrap(self) -> HashMap<String, f32> {
        match self {
            OprsOwned::Reliable(dat) => dat,
            OprsOwned::Unreliable(dat) => dat,
        }
    }
}

pub fn calc_oprs_for_matches(mut matches: Vec<Match>) -> Result<OprsOwned, Error> {
    let mut ignore_teams = Vec::new();

    loop {
        let mut matches = matches.iter_mut().peekable();
        matches.peek().ok_or(Error::NoMatches)?;
        let (mut scores, decomp, num_usable_matches, mut teams, usable_matches) = {
            trace!("Collecting usable matches...");
            let mut usable_matches = matches.filter(|m| {
                m.score_breakdown.is_some()
                && m.alliances.is_some()
            }).collect::<Vec<_>>();

            let num_usable_matches = usable_matches.len();

            trace!("Getting all teams...");
            let mut teams = itertools::Itertools::unique(usable_matches.iter_mut().map(|a_match| {
                a_match.team_keys_mut().unwrap_or_else(Vec::new).into_iter()
            }).flatten().map(|team| mem::replace(team, String::new()))).collect::<Vec<String>>();

            let matrix = {
                trace!("Building rows for matrix...");
                let rows = usable_matches.iter()
                    .flat_map(|a_match| {
                        vec![
                            RowVectorN::<f32, Dynamic>::from_iterator(teams.len(), teams.iter().map(|team| {
                                if a_match.alliances.as_ref().unwrap().red.team_keys.contains(team) {
                                    1.0f32
                                }else{
                                    0f32
                                }
                            })),
                            RowVectorN::<f32, Dynamic>::from_iterator(teams.len(), teams.iter().map(|team| {
                                if a_match.alliances.as_ref().unwrap().blue.team_keys.contains(team) {
                                    1.0f32
                                }else{
                                    0f32
                                }
                            }))
                        ].into_iter()
                    }).collect::<Vec<_>>();
            trace!("Building matrix from rows...");
            DMatrix::from_rows(&rows[..])
        };

        trace!("Built matrix of size {}x{}", matrix.nrows(), matrix.ncols());
            trace!("Building vector...");
            let scores = VectorN::<f32, Dynamic>::from_iterator(usable_matches.len() * 2, usable_matches.iter().flat_map(|a_match| {
                vec![
                    a_match.alliances.as_ref().unwrap().red.score as f32,
                    a_match.alliances.as_ref().unwrap().blue.score as f32
                ].into_iter()
            }));

            trace!("Multiplying by the transpose...");
            let trans = matrix.transpose();
            let matrix = trans.clone() * matrix;
            trace!("Decomposing matrix...");
            (trans * scores, matrix.qr(), num_usable_matches, teams, usable_matches)
        };

        debug!("Solving...");
        if decomp.solve_mut(&mut scores) {
            debug!("Collecting results...");
            let solution = scores;
            let is_reliable = (num_usable_matches as f32 / teams.len() as f32) > 0.75;


            let data = teams.into_iter().zip(solution.into_iter()).map(|(s, v)| (s, *v)).collect();

            break Ok(if is_reliable {
                OprsOwned::Reliable(data)
            } else {
                OprsOwned::Unreliable(data)
            });
        } else {
            debug!("Calculation failed, removing least present team...");

            
            let index = {
                let least_team = teams.iter().min_by_key(|team| {
                    usable_matches.iter().filter(|a_match| a_match.team_keys().unwrap().contains(team)).count()
                }).ok_or(Error::Unsolvable)?;
                teams.iter().position(|team| team == least_team)
                    .ok_or(Error::Unsolvable)?
            };
            
            ignore_teams.push(teams.remove(index));
        }
    }
}

