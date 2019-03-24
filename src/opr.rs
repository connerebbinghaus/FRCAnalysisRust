
use std::collections::HashMap;

pub fn oprs_from_matches(mut matches: Vec<the_blue_alliance::matches::Match>) -> HashMap<String, f32> {
    matches.retain(|m| m.score_breakdown.is_some());

    let scores: nalgebra::DVector<f32> = nalgebra::DVector::from_iterator(matches.len() * 2,
        matches.iter()
            .flat_map(|m| 
                vec![
                    m.score_breakdown.as_ref().unwrap().total_points(true) as f32, 
                    m.score_breakdown.as_ref().unwrap().total_points(false) as f32
                ].into_iter()
            )
    );
    
    let mut teams_list: Vec<_> = matches.iter().flat_map(|m| m.team_keys().unwrap().into_iter()).collect();
    teams_list.sort_unstable();
    teams_list.dedup();
    let teams_list = teams_list;

    let teams: nalgebra::DMatrix<f32> = nalgebra::DMatrix::from_iterator(teams_list.len(), matches.len() * 2, 
        matches.iter()
        .flat_map(|m| {
            let teams_red = &m.alliances.as_ref().unwrap().red.team_keys;
            let red = teams_list.iter().map(|t| if teams_red.contains(t) {1.0} else {0.0}).collect::<Vec<_>>().into_iter();
            let teams_blue = &m.alliances.as_ref().unwrap().blue.team_keys;
            let blue = teams_list.iter().map(|t| if teams_blue.contains(t) {1.0} else {0.0}).collect::<Vec<_>>().into_iter();
            red.chain(blue)
        })
    ).transpose();

    let transpose = teams.transpose();

    let teams_transpose = transpose.clone() * teams;
    let scores_transpose = transpose * scores;

    let decomp = nalgebra::linalg::QR::new(teams_transpose);
    let solution = decomp.solve(&scores_transpose);
    if let Some(solution) = solution {
        teams_list.into_iter().cloned().zip(solution.into_iter().cloned()).collect()
    } else {
        HashMap::new()
    }

}

pub fn dprs_from_matches(mut matches: Vec<the_blue_alliance::matches::Match>) -> HashMap<String, f32> {
    matches.retain(|m| m.score_breakdown.is_some());

    let scores: nalgebra::DVector<f32> = nalgebra::DVector::from_iterator(matches.len() * 2,
        matches.iter()
            .flat_map(|m| 
                vec![
                    m.score_breakdown.as_ref().unwrap().total_points(false) as f32, 
                    m.score_breakdown.as_ref().unwrap().total_points(true) as f32
                ].into_iter()
            )
    );
    
    let mut teams_list: Vec<_> = matches.iter().flat_map(|m| m.team_keys().unwrap().into_iter()).collect();
    teams_list.sort_unstable();
    teams_list.dedup();
    let teams_list = teams_list;

    let teams: nalgebra::DMatrix<f32> = nalgebra::DMatrix::from_iterator(teams_list.len(), matches.len() * 2, 
        matches.iter()
        .flat_map(|m| {
            let teams_red = &m.alliances.as_ref().unwrap().red.team_keys;
            let red = teams_list.iter().map(|t| if teams_red.contains(t) {1.0} else {0.0}).collect::<Vec<_>>().into_iter();
            let teams_blue = &m.alliances.as_ref().unwrap().blue.team_keys;
            let blue = teams_list.iter().map(|t| if teams_blue.contains(t) {1.0} else {0.0}).collect::<Vec<_>>().into_iter();
            red.chain(blue)
        })
    ).transpose();

    let transpose = teams.transpose();

    let teams_transpose = transpose.clone() * teams;
    let scores_transpose = transpose * scores;

    let decomp = nalgebra::linalg::QR::new(teams_transpose);
    let solution = decomp.solve(&scores_transpose);
    if let Some(solution) = solution {
        teams_list.into_iter().cloned().zip(solution.into_iter().cloned()).collect()
    } else {
        HashMap::new()
    }
}


pub fn ccwms_from_oprs_and_dprs(oprs: HashMap<String, f32>, dprs: HashMap<String, f32>) -> HashMap<String, f32> {
    oprs.into_iter().map(|(t, opr)| (t.clone(), opr - dprs[&t])).collect()
}