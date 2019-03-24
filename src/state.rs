
use std::cmp::Ordering;
use std::collections::HashMap;
use futures::{Future, Stream};
use chrono::TimeZone;
pub struct EventState {
    pub ranking: Vec<TeamState>,
}

impl EventState {
    pub fn new<T: chrono::TimeZone>(event: &the_blue_alliance::event::Event, tba: &the_blue_alliance::TBA, time: chrono::DateTime<T>) -> EventState {
        let mut teams: HashMap<String, TeamState> = HashMap::new();
        
        event.matches(tba).into_stream()
            .map(|ms| futures::stream::iter_ok::<_, the_blue_alliance::Error>(ms.into_iter()))
            .flatten()
            .filter(|m| chrono::Utc.timestamp(m.actual_time.or(m.time).unwrap() as i64, 0) < time.with_timezone(&chrono::Utc))
            .filter(|m| m.score_breakdown.is_some())
            .filter(|m| m.comp_level == the_blue_alliance::matches::CompLevel::QualificationMatch)
            .for_each(|m| {
                for t in m.team_keys().unwrap() {
                    teams.entry(t.clone()).or_insert_with(|| TeamState::new(&t)).add_match(m.clone());
                }
                futures::future::ok(())
            }).wait().unwrap();

        let mut teams_vec: Vec<_> = teams.into_iter().map(|(_, v)| v).collect();

        teams_vec.sort_by_key(|t| t.ranking.clone());
        teams_vec.reverse();
        EventState{
            ranking: teams_vec,
        }
    }

    pub fn rank_of_team(&self, team: &str) -> Option<usize> {
        self.ranking.iter().enumerate().find(|(_, t)| t.team == team).map(|(i, _)| i+1)
    }

    pub fn team_data(&self, team: &str) -> Option<TeamRankingData> {
        self.ranking.iter().find(|t| t.team == team).map(|t| t.ranking.clone())
    }

}

pub struct TeamState {
    pub team: String,
    pub ranking: TeamRankingData,
}

impl TeamState {
    fn new(team: &str) -> TeamState {
        TeamState {
            team: team.to_owned(),
            ranking: TeamRankingData::None,
        }
    }

    fn add_match(&mut self, m: the_blue_alliance::matches::Match) {
        self.ranking.accumulate(m, &self.team)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum TeamRankingData {
    None,
    S2019(TeamRankingData2019),
}

impl PartialOrd for TeamRankingData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (TeamRankingData::S2019(ref d), TeamRankingData::S2019(ref d2)) => d.partial_cmp(&d2),
            _ => panic!()
        }
    }
}


impl Ord for TeamRankingData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TeamRankingData {
    fn new() -> TeamRankingData {
        TeamRankingData::None
    }

    fn accumulate(&mut self, data: the_blue_alliance::matches::Match, team: &str) {
        let is_red = if data.alliances.as_ref().unwrap().red.team_keys.contains(&team.to_owned()) {
            true
        } else if data.alliances.as_ref().unwrap().blue.team_keys.contains(&team.to_owned()) {
            false
        } else {
            panic!("Match does not contain team!");
        };
        
        match data.score_breakdown.expect("Match does not have score breakdown") {
            the_blue_alliance::matches::ScoreBreakdown::Year2019(data) => {
                let data = if is_red {
                    data.red
                } else {
                    data.blue
                };

                if let TeamRankingData::S2019(ref mut d) = self {
                    d.played += 1;
                    d.ranking_points += data.rp as u32;
                    d.cargo_points += data.cargo_points as u32;
                    d.panel_points += data.hatch_panel_points as u32;
                    d.climb_points += data.hab_climb_points as u32;
                    d.sandstorm_points += data.sand_storm_bonus_points as u32;
                    d.ranking_score = d.ranking_points as f32 / d.played as f32;
                } else if let TeamRankingData::None = self {
                    *self = TeamRankingData::S2019(TeamRankingData2019{
                        played: 1,
                        ranking_points: data.rp as u32,
                        cargo_points: data.cargo_points as u32,
                        panel_points: data.hatch_panel_points as u32,
                        climb_points: data.hab_climb_points as u32,
                        sandstorm_points: data.sand_storm_bonus_points as u32,
                        ranking_score: data.rp as f32,
                    });
                }
            },
            _ => panic!("Cannot handle data for this season.")
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct TeamRankingData2019 {
    pub ranking_score: f32,
    pub cargo_points: u32,
    pub panel_points: u32,
    pub climb_points: u32,
    pub sandstorm_points: u32,
    pub played: u32,
    pub ranking_points: u32,
}

impl PartialOrd for TeamRankingData2019 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.ranking_score - other.ranking_score).abs() > 0.001 {
            self.ranking_score.partial_cmp(&other.ranking_score)
        } else if self.cargo_points != other.cargo_points {
            Some(self.cargo_points.cmp(&other.cargo_points))
        } else if self.panel_points != other.panel_points {
            Some(self.panel_points.cmp(&other.panel_points))
        } else if self.climb_points != other.climb_points {
            Some(self.climb_points.cmp(&other.climb_points))
        } else {
            Some(self.sandstorm_points.cmp(&other.sandstorm_points))
        }
    }
}

impl Eq for TeamRankingData2019 {}

impl Ord for TeamRankingData2019 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}