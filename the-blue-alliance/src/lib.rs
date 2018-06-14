extern crate hyper;
extern crate log;
extern crate hyper_tls;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate chrono;

use futures::future;
use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper::Request;
use serde::de::Deserialize;
use hyper::rt::{Future, Stream};
use std::result;

pub mod matches;
pub mod team;
pub mod event;
pub mod district;
mod cache;

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
/// let tba = TBA::new("xxxxxxxx"); // Get API key from TBA account page.
/// let team = Team::from_key(tba, "frc4453");
/// assert_eq!(team.number, 4453);
/// ```
pub struct TBA {
    auth_key: &'static str,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    event_loop: tokio_core::reactor::Core
}

impl TBA {
    /// Creates a new TBA struct from an TBA auth key.
    pub fn new(auth_key: &'static str) -> TBA {
        TBA {
            auth_key,
            client: Client::builder().build(HttpsConnector::new(4).expect("Cannot create HttpsConnector")),
            event_loop: tokio_core::reactor::Core::new().expect("Cannot create tokio event loop")
        }
    }

    /// Downloads JSON from the specified TBA api path, deserializing it into type `T`.
    /// Used internally.
    fn get<T>(&mut self, url: String) -> Result<T>
        where for<'de> T: serde::Deserialize<'de>
    {
        let request: Request<hyper::Body> = Request::builder()
            .method(hyper::Method::GET)
            .uri(String::from(BASE_URL) + &url)
            .header("X-TBA-Auth-Key", self.auth_key)
            .body(hyper::Body::empty()).expect("Failed to construct request.");
        let fut = self.client.request(request).map_err(|e| Error::Hyper(e))
            .and_then(|res| {
                println!("Response: {}", res.status());

                res.into_body().map_err(|e| Error::Hyper(e))
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        future::ok::<_, Error>(v)
                    }).and_then(|chunks| {
                        println!("Data: {}", String::from_utf8(chunks.clone()).unwrap());
                        future::result::<_, Error>(TBA::parse_json(chunks).map_err(|e| Error::Json(e)))
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