use ::{TBA, Result};
use ::district::District;
use ::team::Team;
use ::matches::Match;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
pub enum WebcastType{
    #[serde(rename = "youtube")]
    YouTube,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "ustream")]
    Ustream,
    #[serde(rename = "iframe")]
    Iframe,
    #[serde(rename = "html5")]
    Html5,
    #[serde(rename = "rtmp")]
    Rtmp,
    #[serde(rename = "livestream")]
    Livestream
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Webcast {
    #[serde(rename = "type")]
    pub cast_type: WebcastType,
    pub channel: String,
    pub file: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub key: String,
    pub name: String,
    pub event_code: String,
    pub event_type: i32,
    pub district: Option<District>,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub year: i32,
    pub short_name: Option<String>,
    pub event_type_string: String,
    pub week: Option<i32>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub gmaps_place_id: Option<String>,
    pub gmaps_url: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub location_name: Option<String>,
    pub timezone: Option<String>,
    pub website: Option<String>,
    pub first_event_id: Option<String>,
    pub first_event_code: Option<String>,
    pub webcasts: Option<Vec<Webcast>>,
    pub division_keys: Option<Vec<String>>,
    pub parent_event_key: Option<String>,
    pub playoff_type: Option<i32>,
    pub playoff_type_string: Option<String>
}

impl Event {
    pub fn from_key(mut tba: TBA, key: &str) -> Result<Vec<Event>> {
        tba.get("/event/".to_owned() + key)
    }

    pub fn for_team_key(mut tba: TBA, team_key: &str) -> Result<Vec<Event>> {
        tba.get("/team/".to_owned() + team_key + "/events")
    }

    pub fn in_year(mut tba: TBA, year: i32) -> Result<Vec<Event>> {
        assert_eq!(year.to_string().len(), 4);
        tba.get("/events/".to_owned() + &year.to_string())
    }

    pub fn teams(&self, mut tba: TBA) -> Result<Vec<Team>>{
        tba.get("/event/".to_owned() + &self.key + "/teams")
    }

    pub fn matches(&self, mut tba: TBA) -> Result<Vec<Match>>{
        tba.get("/event/".to_owned() + &self.key + "/matches")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventSimple {
    pub key: String,
    pub name: String,
    pub event_code: String,
    pub event_type: i32,
    pub district: Option<District>,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub year: i32
}