use ::team::Team;
use ::district::District;
use ::event::Event;
use ::matches::Match;
use ::chrono::{DateTime, Utc, Local};


pub enum Query {
    Team,
    Match,
    Event,
    District,

    FromKey(String, Query),

    Paginated(u32, Query),

    InYear(u32, Query),

    DistrictsFor(Query),
    EventsFor(Query),
    MatchesFor(Query),
    TeamsFor(Query),

    Simple(Query),
    Keys(Query),
}

pub enum CachedData {
    Team(Team),
    District(District),
    Event(Event),
    Match(Match)
}

struct CachedDataTimed {
    data: CachedData,
    last_modified: DateTime<Local>,
    expires: DateTime<Local>
}

impl CachedDataTimed {
    fn cache(cacheable: &ToCache, last_modified: DateTime<Local>, expires: DateTime<Local>) -> CachedDataTimed {
        let data= cacheable.cache();
        CachedDataTimed {
            data,
            last_modified,
            expires,
        }
    }
}



pub struct CacheStore {

}


pub trait ToCache {
    fn cache(&self) -> CachedData;
    fn all_queries_for<T>(&self, parent: Option<T>) -> Vec<Query>;
}

impl ToCache for Team {
    fn cache(&self) -> CachedData {
        CachedData::Team(*self)
    }

    fn all_queries_for<T>(&self, parent: Option<T>) -> Vec<Query> {
        vec![
            Query::FromKey(self.key.clone(), Query::Team),

        ]
    }
}

impl ToCache for District {
    fn cache(&self) -> CachedData {
        CachedData::District(*self)
    }
}

impl ToCache for Event {
    fn cache(&self) -> CachedData {
        CachedData::Event(*self)
    }
}

impl ToCache for Match {
    fn cache(&self) -> CachedData {
        CachedData::Match(*self)
    }
}