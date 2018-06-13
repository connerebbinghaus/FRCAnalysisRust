
#[derive(Serialize, Deserialize)]
pub struct MatchAlliance {
    score: i32,
    team_keys: [String; 3],
    surrogate_team_keys: Option<Vec<String>>,
    dq_team_keys: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Alliances {
    red: MatchAlliance,
    blue: MatchAlliance
}