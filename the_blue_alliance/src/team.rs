
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchAlliance {
    pub score: i32,
    pub team_keys: [String; 3],
    pub surrogate_team_keys: Option<Vec<String>>,
    pub dq_team_keys: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alliances {
    pub red: MatchAlliance,
    pub blue: MatchAlliance
}