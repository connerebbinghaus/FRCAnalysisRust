use ::TBA;
use ::district::District;
use ::event::Event;
use std::collections::HashMap;
use hyper::rt::Future;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchAlliance {
    pub score: i32,
    pub team_keys: [String; 3],
    pub surrogate_team_keys: Option<Vec<String>>,
    pub dq_team_keys: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alliances {
    pub red: MatchAlliance,
    pub blue: MatchAlliance
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub key: String,
    pub team_number: i32,
    pub nickname: Option<String>,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub gmaps_place_id: Option<String>,
    pub gmaps_url: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub location_name: Option<String>,
    pub website: Option<String>,
    pub rookie_year: i32,
    pub motto: Option<String>,
    pub home_championship: Option<HashMap<String, String>>
}

impl Team {
    pub fn from_key(tba: &TBA, key: &str) -> impl Future<Item = Team, Error = ::Error> {
        tba.clone().get("/team/".to_owned() + key)
    }
    pub fn all(tba: &TBA, page: u32) -> impl Future<Item = Vec<Team>, Error = ::Error> {
        tba.clone().get("/teams/".to_owned() + &page.to_string())
    }

    pub fn in_year(tba: &TBA, year:u32, page: u32) -> impl Future<Item = Vec<Team>, Error = ::Error> {
        assert_eq!(year.to_string().len(), 4);
        tba.clone().get("/teams/".to_owned() + &year.to_string() + "/" +& page.to_string())
    }

    pub fn years_participated(&self, tba: &TBA) -> impl Future<Item = Vec<u32>, Error = ::Error> {
        tba.clone().get("/team/".to_owned() + &self.key + "/years_participated")
    }

    pub fn districts(&self, tba: &TBA) -> impl Future<Item = Vec<District>, Error = ::Error> {
        tba.clone().get("/team/".to_owned() + &self.key + "/districts")
    }

    pub fn events(&self, tba: &TBA) -> impl Future<Item = Vec<Event>, Error = ::Error> {
        tba.clone().get("/team/".to_owned() + &self.key + "/events")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamSimple {
    pub key: String,
    pub number: i32,
    pub nickname: Option<String>,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>
}