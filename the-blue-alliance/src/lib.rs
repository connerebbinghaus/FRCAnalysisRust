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
extern crate serde_cbor;

use futures::future;
use hyper_tls::HttpsConnector;
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
use std::rc::Rc;

const BASE_URL: &str = "https://www.thebluealliance.com/api/v3";

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Json(serde_json::Error),
    CacheMiss
}

/// Stores the TBA auth key, HTTP client, and tokio event loop for use in requesting data from the api.
/// # Examples
/// ```
/// extern crate the_blue_alliance;
/// use the_blue_alliance::TBA;
/// use the_blue_alliance::team::Team;
///
/// let mut tba = TBA::new("WG5pUFbRtNL36CLKw071dPf3gdGeT16ngwuPTWhkQev1pvX2enVnf2hq2oPYtjCH"); // Get API key from TBA account page.
/// let team = Team::from_key(&mut tba, "frc4453");
/// assert_eq!(team.unwrap().team_number, 4453);
/// ```
pub struct TBARaw {
    auth_key: &'static str,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    cache: RwLock<cache::CacheStore>,
}

#[derive(Clone)]
pub struct TBA(Rc<TBARaw>);

impl TBA {
    /// Creates a new TBA struct from a TBA auth key.
    pub fn new(auth_key: &'static str) -> TBA {
        TBA(Rc::new(TBARaw {
            auth_key,
            client: Client::builder().build(HttpsConnector::new(4).expect("Cannot create HttpsConnector")),
            cache: RwLock::new(cache::CacheStore::new()),
        }))
    }

    /// Downloads JSON from the specified TBA api path, deserializing it into type `T`.
    /// Used internally.
    fn get<T: 'static>(self, url: String) ->impl Future<Item = T, Error = Error>
        where for<'de> T: serde::Deserialize<'de>,
                cache::CachedData: cache::ToInternal<T>,
                T: cache::ToCache
    {
        let cache_future = match self.0.cache.read().unwrap().query(url.clone()) {
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

        let request: Request<hyper::Body> = match self.0.cache.read().unwrap().query(url.clone()) {
            None => Request::builder()
                .method(hyper::Method::GET)
                .uri(String::from(BASE_URL) + &url)
                .header("X-TBA-Auth-Key", self.0.auth_key).body(hyper::Body::empty()),
            Some(ref cache) => Request::builder()
                .method(hyper::Method::GET)
                .uri(String::from(BASE_URL) + &url)
                .header("X-TBA-Auth-Key", self.0.auth_key)
                .header::<&str, &str>("If-Modified-Since", cache.last_modified.clone().as_ref()).body(hyper::Body::empty()),
        }.expect("Failed to construct request.");

        debug!("Headers: {:?}", request.headers());

        let (mut head_keep, _) = http::response::Response::builder().body(hyper::body::Body::empty()).expect("Cannot create empty response to initialize head_keep.").into_parts();

        let request_future = self.0.client.request(request).map_err(|e| Error::Hyper(e))
            .and_then(move |res| {
                debug!("Response: {}", res.status());
                if res.status() == 304 {
                    return Box::new(future::ok(self.0.cache.read().unwrap().query(url.clone()).unwrap().data.clone().into_internal()))
                        as Box<future::Future<Item = T, Error = Error>>;
                }
                let (head, body) = res.into_parts();
                head_keep = head;
                Box::new(body.fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, hyper::Error>(v)
                }).map_err(|e| Error::Hyper(e)).and_then(|chunks| {
                    //debug!("Data: {}", String::from_utf8(chunks.clone()).expect("The program crashed while trying to print a debug message, which is stupid."));
                    future::result::<_, Error>(TBA::parse_json(chunks).map_err(|e| Error::Json(e)))
                }).and_then(move |d: T| {
                    self.0.cache.write().unwrap().cache(url, &d, head_keep.headers.get("Last-Modified").expect("Cannot get Last-Modified header.").to_str().expect("Cannot convert Last-Modified header value to string").to_string(), chrono::Local::now() + cache_control::CacheControl::from_value(head_keep.headers.get("Cache-Control").expect("Cannot get Cache-Control header.").to_str().expect("Cannot convert Cache-Control header value to string")).expect("Cannot parse Cache-Control header").max_age.expect("Cache-Control header does not contain max-age value"));
                    future::ok(d)
                })) as Box<future::Future<Item = T, Error = Error>>
            });


        futures::select_ok(vec![Box::new(cache_future) as Box<futures::Future<Item = T, Error = Error>>, Box::new(request_future) as Box<futures::Future<Item = T, Error = Error>>].into_iter())
            .map(|(val, _)| val)
    }

    /// Deserializes the JSON contained in the vector into type `T`.
    /// Used internally by `TBA::get`.
    fn parse_json<T>(body: Vec<u8>) -> serde_json::Result<T>
        where for<'de> T: Deserialize<'de>,
    {
        serde_json::from_slice(&body)
    }
}