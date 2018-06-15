use ::team::Team;
use ::district::District;
use ::event::Event;
use ::matches::Match;
use ::chrono::{DateTime, Utc, Local};
use std::collections::HashMap;


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
    Teams(Vec<Team>),
    District(District),
    Districts(Vec<District>),
    Event(Event),
    Events(Vec<Event>),
    Match(Match),
    Matches(Vec<Match>)
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
    store: HashMap<Query, CachedDataTimed>
}

impl CacheStore {
    pub fn cache(&mut self, query: Query, data: &ToCache) {
        self.store.insert(query, CachedDataTimed::cache(data));
    }

    pub fn query(&self, query: Query) -> Option<&ToCache> {
        if Some(dat) = self.store.get(&query) {

        }
    }
}


pub trait ToCache: Clone {
    fn cache(self) -> CachedData;
}

impl ToCache for Team {
    fn cache(self) -> CachedData {
        CachedData::Team(self)
    }
}

impl ToCache for Vec<Team> {
    fn cache(self) -> CachedData {
        CachedData::Teams(self)
    }
}

impl ToCache for District {
    fn cache(self) -> CachedData {
        CachedData::District(self)
    }
}

impl ToCache for Vec<District> {
    fn cache(self) -> CachedData {
        CachedData::Districts(self)
    }
}

impl ToCache for Event {
    fn cache(self) -> CachedData {
        CachedData::Event(self)
    }
}

impl ToCache for Vec<Event> {
    fn cache(self) -> CachedData {
        CachedData::Events(self)
    }
}

impl ToCache for Match {
    fn cache(&self) -> CachedData {
        CachedData::Match(*self)
    }
}

impl ToCache for Vec<Match> {
    fn cache(self) -> CachedData {
        CachedData::Matches(self)
    }
}