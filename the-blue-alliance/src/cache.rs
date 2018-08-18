use ::team::Team;
use ::district::District;
use ::event::Event;
use ::matches::Match;
use ::chrono::{DateTime, Local};
use std::collections::HashMap;
use serde_cbor;
use std::fs::File;
#[derive(Serialize, Deserialize, Clone)]
pub enum CachedData {
    Team(Box<Team>),
    Teams(Vec<Team>),
    District(District),
    Districts(Vec<District>),
    Event(Box<Event>),
    Events(Vec<Event>),
    Match(Box<Match>),
    Matches(Vec<Match>),
    Years(Vec<u32>),
    Keys(Vec<String>),
}

pub trait ToInternal<T> {
    fn into_internal(self) -> T;
}

impl ToInternal<Box<Team>> for CachedData {
    fn into_internal(self) -> Box<Team> {
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

impl ToInternal<Box<Event>> for CachedData {
    fn into_internal(self) -> Box<Event> {
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

impl ToInternal<Box<Match>> for CachedData {
    fn into_internal(self) -> Box<Match> {
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

#[derive(Serialize, Deserialize)]
pub struct CachedDataTimed {
    pub data: CachedData,
    pub last_modified: String,
    pub expires: DateTime<Local>
}

impl CachedDataTimed {
    fn cache<C: ToCache>(cacheable: C, last_modified: String, expires: DateTime<Local>) -> CachedDataTimed {
        let data= cacheable.cache();
        CachedDataTimed {
            data,
            last_modified,
            expires,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CacheStore {
    store: HashMap<String, CachedDataTimed>
}

impl CacheStore {
    pub fn new() -> CacheStore {
        info!("Loading cache...");
        match File::open("cache.bin") {
            Err(e) => {
                warn!("Cannot load cache file: {}", e);
                None
            },
            Ok(file) => match serde_cbor::from_reader(file) {
                Ok(v) => v,
                Err(e) => {
                    warn!("Cannot deserialize cache data: {}", e);
                    None
                },
            },
        }.or_else(|| {
            Some(CacheStore {
                store: HashMap::new(),
            })
        }).unwrap()
    }

    pub fn cache<C: ToCache>(&mut self, query: String, data: C, last_modified: String, expires: DateTime<Local>) {
        self.store.insert(query, CachedDataTimed::cache(data, last_modified, expires));
    }

    pub fn query(&self, query: &str) -> Option<&CachedDataTimed> {
        self.store.get(query)
    }
}

impl Drop for CacheStore {
    fn drop(&mut self) {
        info!("Saving cache data.");
        let mut file = File::create("cache.bin").unwrap();
        serde_cbor::to_writer(&mut file, &self).expect("Failed to serialize cache");
    }
}

pub trait ToCache {
    fn cache(self) -> CachedData;
}

impl ToCache for Box<Team> {
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

impl ToCache for Box<Event> {
    fn cache(self) -> CachedData {
        CachedData::Event(self)
    }
}

impl ToCache for Vec<Event> {
    fn cache(self) -> CachedData {
        CachedData::Events(self)
    }
}

impl ToCache for Box<Match> {
    fn cache(self) -> CachedData {
        CachedData::Match(self)
    }
}

impl ToCache for Vec<Match> {
    fn cache(self) -> CachedData {
        CachedData::Matches(self)
    }
}

impl ToCache for Vec<u32> {
    fn cache(self) -> CachedData {
        CachedData::Years(self)
    }
}

impl ToCache for Vec<String> {
    fn cache(self) -> CachedData {
        CachedData::Keys(self)
    }
}