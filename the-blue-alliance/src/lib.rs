#![warn(unused_extern_crates)]

extern crate hyper;
#[macro_use]
extern crate log;
extern crate hyper_rustls;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate cache_control;
extern crate http;
extern crate serde_cbor;
extern crate smallvec;
extern crate time;

use futures::future;
use hyper_rustls::HttpsConnector;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper::Request;
use serde::de::Deserialize;
use hyper::rt::{Future, Stream};

pub mod matches;
pub mod team;
pub mod event;
pub mod district;
mod cache;

use cache::ToInternal;
use std::sync::RwLock;
use std::sync::Arc;
use futures::future::Executor;
use smallvec::SmallVec;

const BASE_URL: &str = "https://www.thebluealliance.com/api/v3";

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Json(serde_json::Error),
    CacheMiss,
    Other(&'static str)
}


/// Stores the TBA auth key, HTTP client, and tokio event loop for use in requesting data from the api.
pub struct TBARaw {
    auth_key: &'static str,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    cache: RwLock<cache::CacheStore>,
}

/// Stores the TBA auth key, HTTP client, and tokio event loop for use in requesting data from the api.
#[derive(Clone)]
pub struct TBA(Arc<TBARaw>);

impl TBA {
    /// Creates a new TBA struct from a TBA auth key.
    pub fn new<E>(auth_key: &'static str, exe: E) -> TBA
        where E: Executor<Box<Future<Item=(), Error=()> + Send>> + Send + Sync + 'static,
    {
        TBA(Arc::new(TBARaw {
            auth_key,
            client: Client::builder()
                .executor(exe)
                .build(HttpsConnector::new(4)),
            cache: RwLock::new(cache::CacheStore::new()),
        }))
    }

    /// Downloads JSON from the specified TBA api path, deserializing it into type `T`.
    /// Used internally.
    fn get<T: 'static>(&self, url: String) -> impl future::Future<Error = Error, Item = T> + Send
        where for<'de> T: serde::Deserialize<'de>,
                cache::CachedData: cache::ToInternal<T>,
                T: cache::ToCache,
                T: Send + Sync + Clone,
    {
        let tba = self.0.clone();

        let cache_future = match tba.cache.read().unwrap().query(url.as_str()) {
            None => {
                debug!("Cache MISS for {}", &url);
                futures::future::err(Error::CacheMiss)
            },
            Some(c) => {
                if c.expires > chrono::Local::now() {
                    debug!("Cache HIT for {}", &url);
                    futures::future::ok(c.data.clone().into_internal())
                } else {
                    debug!("Cache EXPIRED for {}", &url);
                    futures::future::err(Error::CacheMiss)
                }
            },
        };

        let request: Request<hyper::Body> = match tba.cache.read().unwrap().query(url.as_str()) {
            None => Request::builder()
                .method(hyper::Method::GET)
                .uri(String::from(BASE_URL) + &url)
                .header("X-TBA-Auth-Key", tba.auth_key).body(hyper::Body::empty()),
            Some(ref cache) => Request::builder()
                .method(hyper::Method::GET)
                .uri(String::from(BASE_URL) + &url)
                .header("X-TBA-Auth-Key", tba.auth_key)
                .header::<&str, &str>("If-Modified-Since", cache.last_modified.clone().as_ref()).body(hyper::Body::empty()),
        }.expect("Failed to construct request.");

        debug!("Headers: {:?}", request.headers());

        let (mut head_keep, _) = http::response::Response::builder().body(hyper::body::Body::empty()).expect("Cannot create empty response to initialize head_keep.").into_parts();

        let request_future = tba.client.request(request).map_err(Error::Hyper)
            .and_then(move |res| {
                debug!("Response: {}", res.status());
                let unmodified_future = if res.status() == 304 {
                    tba.cache.write().unwrap().refresh(url.as_str());
                    future::ok(tba.cache.read().unwrap().query(url.as_str()).unwrap().data.clone().into_internal())
                } else { future::err(Error::Other("Should not occur")) };

                let (head, body) = res.into_parts();
                head_keep = head;

                let full_request_future = body.fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, hyper::Error>(v)
                }).map_err(Error::Hyper).and_then(|chunks| {
                    //debug!("Data: {}", String::from_utf8(chunks.clone()).expect("The program crashed while trying to print a debug message, which is stupid."));
                    future::result::<_, Error>(TBA::parse_json(&chunks).map_err(Error::Json))
                }).and_then(move |d: T| {
                    tba.cache.write().unwrap().cache(url, d.clone(), head_keep.headers.get("Last-Modified").expect("Cannot get Last-Modified header.").to_str().expect("Cannot convert Last-Modified header value to string").to_string(), cache_control::CacheControl::from_value(head_keep.headers.get("Cache-Control").expect("Cannot get Cache-Control header.").to_str().expect("Cannot convert Cache-Control header value to string")).expect("Cannot parse Cache-Control header").max_age.expect("Cache-Control header does not contain max-age value"));
                    future::ok(d)
                });

                let mut futures: SmallVec<[_; 2]> = SmallVec::new();
                futures.push(future::Either::A(unmodified_future));
                futures.push(future::Either::B(full_request_future));
                future::select_ok(futures)
                    .map(|(res, _others)| res)
            });


        let mut futures: SmallVec<[_; 2]> = SmallVec::new();
        futures.push(future::Either::A(cache_future));
        futures.push(future::Either::B(request_future));
        future::select_ok(futures)
            .map(|(res, _others)| res)
    }

    /// Deserializes the JSON contained in the vector into type `T`.
    /// Used internally by `TBA::get`.
    fn parse_json<T>(body: &[u8]) -> serde_json::Result<T>
        where for<'de> T: Deserialize<'de>,
    {
        serde_json::from_slice(&body)
    }
}