use nalgebra::*;
use itertools::{chain, Itertools};
use the_blue_alliance::matches::Match;

pub fn calc_oprs_for_matches(matches: Vec<Match>) -> Option<Vec<(String, f32)>> {
    debug!("Collecting usable matches...");
    let usable_matches = matches.iter().filter(|m| {
        m.score_breakdown.is_some()
        && m.alliances.is_some()
    }).collect::<Vec<_>>();

    debug!("Getting all teams...");
    let teams = usable_matches.iter().flat_map(|a_match| {
        chain(a_match.alliances.as_ref().unwrap().red.team_keys.iter(),
            a_match.alliances.as_ref().unwrap().blue.team_keys.iter())
    }).unique().collect::<Vec<&String>>();

    debug!("Building rows for matrix...");
    let rows = usable_matches.iter()
        .flat_map(|a_match| {
            vec![
                RowVectorN::<f32, Dynamic>::from_iterator(teams.len(), teams.iter().map(|team| {
                    match a_match.alliances.as_ref().unwrap().red.team_keys.contains(team) {
                        true => 1.0f32,
                        false => 0f32
                    }
                })),
                RowVectorN::<f32, Dynamic>::from_iterator(teams.len(), teams.iter().map(|team| {
                    match a_match.alliances.as_ref().unwrap().blue.team_keys.contains(team) {
                        true => 1.0f32,
                        false => 0f32
                    }
                }))
                ].into_iter()
        }).collect::<Vec<_>>();


    debug!("Building matrix from rows...");
    let matrix = DMatrix::from_rows(&rows[..]);

    debug!("Built matrix of size {}x{}", matrix.nrows(), matrix.ncols());

    debug!("Building vector...");
    let scores = VectorN::<f32, Dynamic>::from_iterator(usable_matches.len() * 2, usable_matches.into_iter().flat_map(|a_match| {
        vec![
            a_match.alliances.as_ref().unwrap().red.score as f32,
            a_match.alliances.as_ref().unwrap().blue.score as f32
        ].into_iter()
    }));

    debug!("Multiplying by the transpose...");
    let trans = matrix.transpose();
    let matrix = trans.clone() * matrix;
    let scores = trans * scores;

    debug!("Decomposing matrix...");
    let decomp = matrix.qr();
    debug!("Solving...");
    match decomp.solve(&scores) {
        Some(solution) => Some(teams.into_iter().zip(solution.into_iter()).map(|(s, v)| (s.clone(), v.clone())).collect()),
        None => {
            warn!("Cannot find oprs for matches: Cannot solve.");
            None
        }

    }
}
