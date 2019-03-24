use std::cmp::Ordering;
use crate::TBA;
use crate::district::District;
use crate::team::Team;
use crate::matches::Match;
use chrono::NaiveDate;
use futures::future;
use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    Livestream,
    #[serde(rename = "dacast")]
    DaCast,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Webcast {
    #[serde(rename = "type")]
    pub cast_type: WebcastType,
    pub channel: String,
    pub file: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn from_key(tba: &TBA, key: &str) -> impl future::Future<Error = Error, Item = Box<Event>> + Send{
        tba.get("/event/".to_owned() + key)
    }

    pub fn for_team_key(tba: &TBA, team_key: &str) -> impl future::Future<Error = Error, Item = Vec<Event>> + Send{
        tba.get("/team/".to_owned() + team_key + "/events")
    }

    pub fn in_year(tba: &TBA, year: i32) -> impl future::Future<Error = Error, Item = Vec<Event>> + Send{
        assert_eq!(year.to_string().len(), 4);
        tba.get("/events/".to_owned() + &year.to_string())
    }

    pub fn teams(&self, tba: &TBA) -> impl future::Future<Error = Error, Item = Vec<Team>> + Send{
        tba.get("/event/".to_owned() + &self.key + "/teams")
    }

    pub fn matches(&self, tba: &TBA) -> impl future::Future<Error = Error, Item = Vec<Match>> + Send{
        tba.get("/event/".to_owned() + &self.key + "/matches")
    }
}

impl PartialEq<Event> for Event {
    fn eq(&self, other: &Event) -> bool {
        self.key == other.key
    }
}

impl Eq for Event {}

impl PartialOrd<Event> for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.start_date.cmp(&other.start_date))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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