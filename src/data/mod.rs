use futures_cpupool;
use futures::stream::iter_ok;
use the_blue_alliance::TBA;
use chrono::TimeZone;
use the_blue_alliance::team::Team;
use the_blue_alliance::event::Event;
use the_blue_alliance::district::District;
use std::marker::PhantomData;
use the_blue_alliance::matches::Match;
use chrono::{DateTime, Utc, Datelike};
use futures::{Future, Stream, sync::oneshot};
use the_blue_alliance::Error;

pub struct Data {
    tba: TBA,
    executor: futures_cpupool::CpuPool,
}

pub enum QuerySelect {
    Oldest,
    Newest,
    All
}

pub enum QueryResult<T> {
    None,
    Single(T),
    Multiple(Vec<T>)
}

impl<T> QueryResult<T> {
    pub fn single(self) -> Option<T> {
        if let QueryResult::Single(dat) = self {
            Some(dat)
        } else {
            None
        }
    }
    
    pub fn multiple(self) -> Option<Vec<T>> {
        if let QueryResult::Multiple(dat) = self {
            Some(dat)
        } else {
            None
        }
    }
}

pub struct QueryBuilder<'a, T> {
    data: &'a Data,
    season: u16,
    key: Option<String>,
    before: Option<DateTime<Utc>>,
    after: Option<DateTime<Utc>>,
    contains_team: Option<String>,
    with_event: Option<String>,
    select: QuerySelect,
    _p: PhantomData<T>
}

impl<'a, T> QueryBuilder<'a, T> {
    fn new(data: &'a Data) -> QueryBuilder<'a, ()> {
        QueryBuilder {
            data: data,
            season: Utc::now().year() as u16,
            key: None,
            before: None,
            after: None,
            contains_team: None,
            with_event: None,
            select: QuerySelect::Newest,
            _p: PhantomData
        }
    }
    
    pub fn in_season(mut self, season: u16) -> Self {
        self.season = season;
        self
    }

    pub fn find_match(self) -> QueryBuilder<'a, Match> {
        QueryBuilder {
            _p: PhantomData,
            data: self.data,
            season: self.season,
            key: None,
            before: self.before,
            after: self.after,
            contains_team: self.contains_team,
            with_event: self.with_event,
            select: self.select
        }
    }

    pub fn find_team(self) -> QueryBuilder<'a, Team> {
        QueryBuilder {
            _p: PhantomData,
            data: self.data,
            season: self.season,
            key: None,
            before: self.before,
            after: self.after,
            contains_team: self.contains_team,
            with_event: self.with_event,
            select: self.select
        }
    }

    pub fn find_event(self) -> QueryBuilder<'a, Event> {
        QueryBuilder {
            _p: PhantomData,
            data: self.data,
            season: self.season,
            key: None,
            before: self.before,
            after: self.after,
            contains_team: self.contains_team,
            with_event: self.with_event,
            select: self.select
        }
    }

    pub fn find_district(self) -> QueryBuilder<'a, District> {
        QueryBuilder {
            _p: PhantomData,
            data: self.data,
            season: self.season,
            key: None,
            before: self.before,
            after: self.after,
            contains_team: self.contains_team,
            with_event: self.with_event,
            select: self.select
        }
    }

    pub fn with_key(mut self, key: &str) -> Self {
        self.key = Some(key.to_owned());
        self
    }

    pub fn choose(mut self, choose: QuerySelect) -> Self {
        self.select = choose;
        self
    }
}

impl<'a> QueryBuilder<'a, Match> {
    pub fn before_date<Tz: TimeZone>(mut self, date: DateTime<Tz>) -> Self {
        self.before = Some(date.with_timezone(&Utc));
        self
    }
    
    pub fn after_date<Tz: TimeZone>(mut self, date: DateTime<Tz>) -> Self {
        self.after = Some(date.with_timezone(&Utc));
        self
    }

    pub fn before_match(self, the_match: &Match) -> Option<Self> {
        let date = Utc.timestamp_opt(the_match.actual_time.or(the_match.predicted_time).or(the_match.time)? as i64, 0).single()?;
        Some(self.before_date(date))
    }
    
    pub fn after_match(self, the_match: &Match) -> Option<Self> {
        let date = Utc.timestamp_opt(the_match.actual_time.or(the_match.predicted_time).or(the_match.time)? as i64, 0).single()?;
        Some(self.after_date(date))
    }

    pub fn has_team_key(mut self, key: &str) -> Self {
        self.contains_team = Some(key.to_owned());
        self
    }
    
    pub fn has_team(self, team: &Team) -> Self {
        let key = team.key.as_str();
        self.has_team_key(key)
    }

    pub fn in_event_key(mut self, key: &str) -> Self {
        self.with_event = Some(key.to_owned());
        self
    }

    pub fn in_event(self, event: &Event) -> Self {
        let key = event.key.as_str();
        self.in_event_key(key)
    }

    pub fn go(self) -> QueryResult<Match> {
        let tba = self.data.tba.clone();
        let events_stream = {
            let tba2 = self.data.tba.clone();
            let tba3 = tba.clone();
            District::in_year(&tba2, self.season as i32).into_stream()
                .map(|districts| iter_ok::<_, Error>(districts.into_iter()))
                .flatten()
                .map(move |district| district.events(&tba3).into_stream())
                .flatten()
                .map(|events| iter_ok::<_, Error>(events.into_iter()))
                .flatten()
        };

        let events_stream = if let Some(event_key) = self.with_event {
            let event_key = event_key.clone();
            events_stream
                .filter(move |event| {
                    event.key == event_key
                }).boxed()
        } else {
            events_stream
                .filter(|_| true)
                .boxed()
        };

        let matches_stream = {
            let tba4 = tba.clone();
            events_stream
                .map(move |event| event.matches(&tba4).into_stream())
                .flatten()
                .map(|matches| iter_ok::<_, Error>(matches.into_iter()))
                .flatten()
        };

        let matches_stream = if let Some(before) = self.before {
            matches_stream
                .filter(move |a_match| {
                    let timestamp = a_match.actual_time.or(a_match.predicted_time).or(a_match.time);
                    if let Some(timestamp) = timestamp {
                        let date = Utc.timestamp_opt(timestamp as i64, 0).single();
                        if let Some(date) = date {
                            date.lt(&before)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }).boxed()
        } else {
            matches_stream.filter(|_| true).boxed()
        };

        let matches_stream = if let Some(after) = self.after {
            matches_stream
                .filter(move |a_match| {
                    let timestamp = a_match.actual_time.or(a_match.predicted_time).or(a_match.time);
                    if let Some(timestamp) = timestamp {
                        let date = Utc.timestamp_opt(timestamp as i64, 0).single();
                        if let Some(date) = date {
                            date.gt(&after)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }).boxed()
        } else {
            matches_stream.filter(|_| true).boxed()
        };

        let matches_stream = if let Some(team) = self.contains_team {
            matches_stream
                .filter(move |a_match| {
                    let teams = a_match.team_keys();
                    if let Some(teams) = teams {
                        teams.contains(&&team)
                    } else {
                        false
                    }
                }).boxed()
        } else {
            matches_stream.filter(|_| true).boxed()
        };

        let matches_stream = if let Some(key) = self.key {
            matches_stream
                .filter(move |a_match| {
                    a_match.key == key
                }).boxed()
        } else {
            matches_stream.filter(|_| true).boxed()
        };

        let select = self.select;

        let result_fut = matches_stream.collect()
            .map(|mut matches| {
                matches.sort();
                let len = matches.len();
                match (len, select)  {
                    (0, _) => QueryResult::None,
                    (1, _) => QueryResult::Single(matches.remove(0)),
                    (_, QuerySelect::Newest) => QueryResult::Single(matches.remove(0)),
                    (_, QuerySelect::Oldest) => QueryResult::Single(matches.remove(len-1)),
                    (_, QuerySelect::All) => QueryResult::Multiple(matches),
                }
            });

        oneshot::spawn(result_fut, &self.data.executor).wait().unwrap()
    }
}

impl<'a> QueryBuilder<'a, Event> {
    pub fn before_date<Tz: TimeZone>(mut self, date: DateTime<Tz>) -> Self {
        self.before = Some(date.with_timezone(&Utc));
        self
    }
    
    pub fn after_date<Tz: TimeZone>(mut self, date: DateTime<Tz>) -> Self {
        self.after = Some(date.with_timezone(&Utc));
        self
    }

    pub fn has_team_key(mut self, key: &str) -> Self {
        self.contains_team = Some(key.to_owned());
        self
    }
    
    pub fn has_team(self, team: &Team) -> Self {
        let key = team.key.as_str();
        self.has_team_key(key)
    }

    pub fn go(self) -> QueryResult<Event> {
        let tba = self.data.tba.clone();
        let events_stream = {
            let tba2 = self.data.tba.clone();
            let tba3 = tba.clone();
            District::in_year(&tba2, self.season as i32).into_stream()
                .map(|districts| iter_ok::<_, Error>(districts.into_iter()))
                .flatten()
                .map(move |district| district.events(&tba3).into_stream())
                .flatten()
                .map(|events| iter_ok::<_, Error>(events.into_iter()))
                .flatten()
        };

        let events_stream = if let Some(before) = self.before {
            events_stream
                .filter(move |event| {
                    let date = Utc.from_utc_date(&event.start_date);
                    date.lt(&before.date())
                }).boxed()
        } else {
            events_stream.filter(|_| true).boxed()
        };

        let events_stream = if let Some(after) = self.after {
            events_stream
                .filter(move |event| {
                    let date = Utc.from_utc_date(&event.start_date);
                    date.gt(&after.date())
                }).boxed()
        } else {
            events_stream.filter(|_| true).boxed()
        };


        let events_stream = if let Some(team) = self.contains_team {
            let tba4 = tba.clone();
            events_stream
                .filter(move |event| {
                    let teams = event.teams(&tba4).wait().ok();
                    if let Some(teams) = teams {
                        teams.iter().find(|t| t.key == team).is_some()
                    } else {
                        false
                    }
                }).boxed()
        } else {
            events_stream.filter(|_| true).boxed()
        };

        let events_stream = if let Some(key) = self.key {
            events_stream
                .filter(move |event| {
                    event.key == key
                }).boxed()
        } else {
            events_stream.filter(|_| true).boxed()
        };

        let select = self.select;

        let result_fut = events_stream.collect()
            .map(|mut events| {
                events.sort();
                let len = events.len();
                match (len, select)  {
                    (0, _) => QueryResult::None,
                    (1, _) => QueryResult::Single(events.remove(0)),
                    (_, QuerySelect::Newest) => QueryResult::Single(events.remove(0)),
                    (_, QuerySelect::Oldest) => QueryResult::Single(events.remove(len-1)),
                    (_, QuerySelect::All) => QueryResult::Multiple(events),
                }
            });

        oneshot::spawn(result_fut, &self.data.executor).wait().unwrap()
    }
}

impl Data {
    pub fn new(tba: TBA) -> Data {
        Data {
            tba,
            executor: futures_cpupool::Builder::new()
                .after_start(|| debug!("Worker thread started."))
                .create()
        }
    }

    pub fn query<'a>(&'a self) -> QueryBuilder<'a, ()> {
        QueryBuilder::<()>::new(self)
    }

    pub fn get_tba(&self) -> TBA {
        self.tba.clone()
    }
}