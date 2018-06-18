extern crate hyper;
#[macro_use]
extern crate log;
extern crate hyper_tls;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate chrono;
extern crate cache_control;
extern crate time;
extern crate http;

use futures::future;
use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper::Request;
use serde::de::Deserialize;
use hyper::rt::{Future, Stream};
use std::result;
use std::borrow::BorrowMut;

pub mod matches;
pub mod team;
pub mod event;
pub mod district;
mod cache;

use cache::ToInternal;

const BASE_URL: &str = "https://www.thebluealliance.com/api/v3";

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Json(serde_json::Error),
}

pub type Result<T> = result::Result<T, Error>;


/// Stores the TBA auth key, HTTP client, and tokio event loop for use in requesting data from the api.
/// # Examples
/// ```
/// extern crate the_blue_alliance;
/// use the_blue_alliance::TBA;
/// use the_blue_alliance::team::Team;
///
/// let tba = TBA::new("xxxxxxxx"); // Get API key from TBA account page.
/// let team = Team::from_key(&mut tba, "frc4453");
/// assert_eq!(team.unwrap().number, 4453);
/// ```
pub struct TBA {
    auth_key: &'static str,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    event_loop: tokio_core::reactor::Core,
    cache: cache::CacheStore,
}

impl TBA {
    /// Creates a new TBA struct from an TBA auth key.
    pub fn new(auth_key: &'static str) -> TBA {
        TBA {
            auth_key,
            client: Client::builder().build(HttpsConnector::new(4).expect("Cannot create HttpsConnector")),
            event_loop: tokio_core::reactor::Core::new().expect("Cannot create tokio event loop"),
            cache: cache::CacheStore::new(),
        }
    }

    /// Downloads JSON from the specified TBA api path, deserializing it into type `T`.
    /// Used internally.
    fn get<T>(&mut self, url: String) -> Result<T>
        where for<'de> T: serde::Deserialize<'de>,
                cache::CachedData: cache::ToInternal<T>,
                T: cache::ToCache
    {
        if let Some(c) = self.cache.query(url.clone()) {
            if c.expires > chrono::Local::now() {
                debug!("Cache HIT for {}", &url);
                return Ok(c.data.clone().into_internal());
            }
        }
        debug!("Cache MISS for {}", &url);




        let request: Request<hyper::Body>  = match self.cache.query(url.clone()) {
            None => Request::builder()
                .method(hyper::Method::GET)
                .uri(String::from(BASE_URL) + &url)
                .header("X-TBA-Auth-Key", self.auth_key).body(hyper::Body::empty()),
            Some(ref cache) => Request::builder()
                .method(hyper::Method::GET)
                .uri(String::from(BASE_URL) + &url)
                .header("X-TBA-Auth-Key", self.auth_key)
                .header::<&str, &str>("Last-Modified", cache.last_modified.clone().as_ref()).body(hyper::Body::empty()),
        }.expect("Failed to construct request.");

        let (mut head_keep, _) = http::response::Response::builder().body(hyper::body::Body::empty()).expect("Cannot create empty response to initialize head_keep.").into_parts();

        let cache_store = self.cache.borrow_mut();

        let fut = self.client.request(request).map_err(|e| Error::Hyper(e))
            .and_then(|res| {
                debug!("Response: {}", res.status());
                let (head, body) = res.into_parts();
                head_keep = head;
                body.fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        future::ok::<_, hyper::Error>(v)
                    }).map_err(|e| Error::Hyper(e)).and_then(|chunks| {
                        debug!("Data: {}", String::from_utf8(chunks.clone()).expect("The program crashed while trying to print a debug message, which is stupid."));
                        future::result::<_, Error>(TBA::parse_json(chunks).map_err(|e| Error::Json(e)))
                    }).and_then(|d: T| {
                    cache_store.cache(url, &d, head_keep.headers.get("Last-Modified").expect("Cannot get Last-Modified header.").to_str().expect("Cannot convert Last-Modified header value to string").to_string(), chrono::Local::now() + cache_control::CacheControl::from_value(head_keep.headers.get("Cache-Control").expect("Cannot get Cache-Control header.").to_str().expect("Cannot convert Cache-Control header value to string")).expect("Cannot parse Cache-Control header").max_age.expect("Cache-Control header does not contain max-age value"));
                        future::ok(d)
                    })
            });
        self.event_loop.run(fut)

    }

    /// Deserializes the JSON contained in the vector into type `T`.
    /// Used internally by `TBA::get`.
    fn parse_json<T>(body: Vec<u8>) -> serde_json::Result<T>
        where for<'de> T: Deserialize<'de>,
    {
        serde_json::from_slice(&body)
    }
}