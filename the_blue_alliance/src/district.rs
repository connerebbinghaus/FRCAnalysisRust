use ::{TBA, Result};
use ::team::{Team, TeamSimple};
use ::event::{Event, EventSimple};

#[derive(Serialize, Deserialize, Debug)]
pub struct District {
    pub abbreviation: String,
    pub display_name: String,
    pub key: String,
    pub year: i32
}

impl District {
    pub fn in_year(mut tba: TBA, year: i32) -> Result<Vec<District>> {
        assert_eq!(year.to_string().len(), 4);
        tba.get("/districts/".to_owned() + &year.to_string())
    }

    pub fn for_team_key(mut tba: TBA, team_key: &str) -> Result<Vec<District>> {
        tba.get("/team/".to_owned() + team_key + "/districts")
    }

    pub fn teams(&self, mut tba: TBA) -> Result<Vec<Team>> {
        tba.get("/district/".to_owned() + &self.key + "/teams")
    }

    pub fn teams_simple(&self, mut tba: TBA) -> Result<Vec<TeamSimple>> {
        tba.get("/district/".to_owned() + &self.key + "/teams/simple")
    }

    pub fn team_keys(&self, mut tba: TBA) -> Result<Vec<String>> {
        tba.get("/district/".to_owned() + &self.key + "/teams/keys")
    }

    pub fn events(&self, mut tba: TBA) -> Result<Vec<Event>> {
        tba.get("/district/".to_owned() + &self.key + "/events")
    }

    pub fn events_simple(&self, mut tba: TBA) -> Result<Vec<EventSimple>> {
        tba.get("/district/".to_owned() + &self.key + "/events/simple")
    }

    pub fn event_keys(&self, mut tba: TBA) -> Result<Vec<String>> {
        tba.get("/district/".to_owned() + &self.key + "/events/keys")
    }
}