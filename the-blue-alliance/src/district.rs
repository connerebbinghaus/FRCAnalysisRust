use ::TBA;
use ::team::Team;
use ::event::Event;
use futures::future;
use Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct District {
    pub abbreviation: String,
    pub display_name: String,
    pub key: String,
    pub year: i32
}

impl District {
    pub fn in_year(tba: &TBA, year: i32) -> impl future::Future<Error = Error, Item = Vec<District>> + Send{
        assert_eq!(year.to_string().len(), 4);
        tba.get("/districts/".to_owned() + &year.to_string())
    }

    pub fn for_team_key(tba: &TBA, team_key: &str) -> impl future::Future<Error = Error, Item = Vec<District>> + Send{
        tba.get("/team/".to_owned() + team_key + "/districts")
    }

    pub fn teams(&self, tba: &TBA) -> impl future::Future<Error = Error, Item = Vec<Team>> + Send{
        tba.get("/district/".to_owned() + &self.key + "/teams")
    }

//    pub fn teams_simple(&self, tba: &mut TBA) -> Result<Vec<TeamSimple>> {
//        tba.get("/district/".to_owned() + &self.key + "/teams/simple")
//    }

    pub fn team_keys(&self, tba: &TBA) -> impl future::Future<Error = Error, Item = Vec<String>> + Send{
        tba.get("/district/".to_owned() + &self.key + "/teams/keys")
    }

    pub fn events(&self, tba: &TBA) -> impl future::Future<Error = Error, Item = Vec<Event>> + Send{
        tba.get("/district/".to_owned() + &self.key + "/events")
    }

//    pub fn events_simple(&self, tba: &mut TBA) -> Result<Vec<EventSimple>> {
//        tba.get("/district/".to_owned() + &self.key + "/events/simple")
//    }

    pub fn event_keys(&self, tba: &TBA) -> impl future::Future<Error = Error, Item = Vec<String>> + Send{
        tba.get("/district/".to_owned() + &self.key + "/events/keys")
    }
}