use ::team::Team;
use ::district::District;
use ::event::Event;
use ::matches::Match;
use ::chrono::{DateTime, Local};
use std::collections::HashMap;


#[derive(Clone)]
pub enum CachedData {
    Team(Team),
    Teams(Vec<Team>),
    District(District),
    Districts(Vec<District>),
    Event(Event),
    Events(Vec<Event>),
    Match(Match),
    Matches(Vec<Match>),
    Years(Vec<u32>),
    Keys(Vec<String>),
}

pub trait ToInternal<T> {
    fn into_internal(self) -> T;
}

impl ToInternal<Team> for CachedData {
    fn into_internal(self) -> Team {
        match self {
            CachedData::Team(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Vec<Team>> for CachedData {
    fn into_internal(self) -> Vec<Team> {
        match self {
            CachedData::Teams(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<District> for CachedData {
    fn into_internal(self) -> District {
        match self {
            CachedData::District(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Vec<District>> for CachedData {
    fn into_internal(self) -> Vec<District> {
        match self {
            CachedData::Districts(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Event> for CachedData {
    fn into_internal(self) -> Event {
        match self {
            CachedData::Event(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Vec<Event>> for CachedData {
    fn into_internal(self) -> Vec<Event> {
        match self {
            CachedData::Events(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Match> for CachedData {
    fn into_internal(self) -> Match {
        match self {
            CachedData::Match(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Vec<Match>> for CachedData {
    fn into_internal(self) -> Vec<Match> {
        match self {
            CachedData::Matches(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Vec<u32>> for CachedData {
    fn into_internal(self) -> Vec<u32> {
        match self {
            CachedData::Years(t) => t,
            _ => panic!(),
        }
    }
}

impl ToInternal<Vec<String>> for CachedData {
    fn into_internal(self) -> Vec<String> {
        match self {
            CachedData::Keys(t) => t,
            _ => panic!(),
        }
    }
}

pub struct CachedDataTimed {
    pub data: CachedData,
    pub last_modified: String,
    pub expires: DateTime<Local>
}

impl CachedDataTimed {
    fn cache(cacheable: &ToCache, last_modified: String, expires: DateTime<Local>) -> CachedDataTimed {
        let data= cacheable.cache();
        CachedDataTimed {
            data,
            last_modified,
            expires,
        }
    }
}


pub struct CacheStore {
    store: HashMap<String, CachedDataTimed>
}

impl CacheStore {
    pub fn new() -> CacheStore {
        CacheStore {
            store: HashMap::new(),
        }
    }

    pub fn cache(&mut self, query: String, data: &ToCache, last_modified: String, expires: DateTime<Local>) {
        self.store.insert(query, CachedDataTimed::cache(data, last_modified, expires));
    }

    pub fn query(&self, query: String) -> Option<&CachedDataTimed> {
        self.store.get(&query)
    }
}


pub trait ToCache {
    fn cache(&self) -> CachedData;
}

impl ToCache for Team {
    fn cache(&self) -> CachedData {
        CachedData::Team(self.clone())
    }
}

impl ToCache for Vec<Team> {
    fn cache(&self) -> CachedData {
        CachedData::Teams(self.clone())
    }
}

impl ToCache for District {
    fn cache(&self) -> CachedData {
        CachedData::District(self.clone())
    }
}

impl ToCache for Vec<District> {
    fn cache(&self) -> CachedData {
        CachedData::Districts(self.clone())
    }
}

impl ToCache for Event {
    fn cache(&self) -> CachedData {
        CachedData::Event(self.clone())
    }
}

impl ToCache for Vec<Event> {
    fn cache(&self) -> CachedData {
        CachedData::Events(self.clone())
    }
}

impl ToCache for Match {
    fn cache(&self) -> CachedData {
        CachedData::Match(self.clone())
    }
}

impl ToCache for Vec<Match> {
    fn cache(&self) -> CachedData {
        CachedData::Matches(self.clone())
    }
}

impl ToCache for Vec<u32> {
    fn cache(&self) -> CachedData {
        CachedData::Years(self.clone())
    }
}

impl ToCache for Vec<String> {
    fn cache(&self) -> CachedData {
        CachedData::Keys(self.clone())
    }
}