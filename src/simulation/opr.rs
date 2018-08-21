use nalgebra::*;
use itertools::{chain, Itertools};
use the_blue_alliance::matches::Match;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Oprs {
    Reliable(HashMap<String, f32>),
    Unreliable(HashMap<String, f32>),
}

impl Oprs {
    pub fn is_reliable(&self) -> bool {
        match self {
            Oprs::Reliable(_) => true,
            Oprs::Unreliable(_) => false,
        }
    }

    pub fn unwrap(self) -> HashMap<String, f32> {
        match self {
            Oprs::Reliable(dat) => dat,
            Oprs::Unreliable(dat) => dat,
        }
    }
}


pub fn calc_oprs_for_matches(matches: &[Match]) -> Option<Oprs> {
    if matches.is_empty() {
        return None;
    }
    trace!("Collecting usable matches...");
    let usable_matches = matches.iter().filter(|m| {
        m.score_breakdown.is_some()
        && m.alliances.is_some()
    }).collect::<Vec<_>>();

    let num_usable_matches = usable_matches.len();

    trace!("Getting all teams...");
    let teams = usable_matches.iter().flat_map(|a_match| {
        chain(a_match.alliances.as_ref().unwrap().red.team_keys.iter(),
            a_match.alliances.as_ref().unwrap().blue.team_keys.iter())
    }).unique().collect::<Vec<&String>>();

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
    let matrix = DMatrix::from_rows(&rows[..]);

    trace!("Built matrix of size {}x{}", matrix.nrows(), matrix.ncols());

    trace!("Building vector...");
    let scores = VectorN::<f32, Dynamic>::from_iterator(usable_matches.len() * 2, usable_matches.into_iter().flat_map(|a_match| {
        vec![
            a_match.alliances.as_ref().unwrap().red.score as f32,
            a_match.alliances.as_ref().unwrap().blue.score as f32
        ].into_iter()
    }));

    trace!("Multiplying by the transpose...");
    let trans = matrix.transpose();
    let matrix = trans.clone() * matrix;
    let scores = trans * scores;

    trace!("Decomposing matrix...");
    let decomp = matrix.qr();
    debug!("Solving...");
    match decomp.solve(&scores) {
        Some(solution) => {
            let is_reliable = (num_usable_matches as f32 / teams.len() as f32) > 0.75;

            let data = teams.into_iter().zip(solution.into_iter()).map(|(s, v)| (s.clone(), *v)).collect();

            Some(if is_reliable {
                Oprs::Reliable(data)
            } else {
                Oprs::Unreliable(data)
            })
        },
        None => {
            //warn!("Cannot calculate oprs for matches: No solution.");
            None
        }

    }
}
