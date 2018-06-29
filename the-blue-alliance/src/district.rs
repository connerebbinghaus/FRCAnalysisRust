use ::TBA;
use ::team::Team;
use ::event::Event;
use hyper::rt::Future;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct District {
    pub abbreviation: String,
    pub display_name: String,
    pub key: String,
    pub year: i32
}

impl District {
    pub fn in_year(tba: &TBA, year: i32) -> impl Future<Item = Vec<District>, Error = ::Error> {
        assert_eq!(year.to_string().len(), 4);
        tba.clone().get("/districts/".to_owned() + &year.to_string())
    }

    pub fn for_team_key(tba: &TBA, team_key: &str) ->  impl Future<Item = Vec<District>, Error = ::Error> {
        tba.clone().get("/team/".to_owned() + team_key + "/districts")
    }

    pub fn teams(&self, tba: &TBA) ->  impl Future<Item = Vec<Team>, Error = ::Error> {
        tba.clone().get("/district/".to_owned() + &self.key + "/teams")
    }

//    pub fn teams_simple(&self, tba: &mut TBA) -> Result<Vec<TeamSimple>> {
//        tba.get("/district/".to_owned() + &self.key + "/teams/simple")
//    }

    pub fn team_keys(&self, tba: &TBA) ->  impl Future<Item = Vec<String>, Error = ::Error> {
        tba.clone().get("/district/".to_owned() + &self.key + "/teams/keys")
    }

    pub fn events(&self, tba: &TBA) ->  impl Future<Item = Vec<Event>, Error = ::Error> {
        tba.clone().get("/district/".to_owned() + &self.key + "/events")
    }

//    pub fn events_simple(&self, tba: &mut TBA) -> Result<Vec<EventSimple>> {
//        tba.get("/district/".to_owned() + &self.key + "/events/simple")
//    }

    pub fn event_keys(&self, tba: &TBA) ->  impl Future<Item = Vec<String>, Error = ::Error> {
        tba.clone().get("/district/".to_owned() + &self.key + "/events/keys")
    }
}