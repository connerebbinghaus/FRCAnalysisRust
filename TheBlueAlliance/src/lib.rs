#![feature(custom_attribute)]

#[macro_use]
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate bytes;
#[macro_use]
extern crate serde_derive;

use futures::future;
use futures::executor::ThreadPool;
use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::{Request, Response};
use hyper::header::HeaderMap;
use serde::de::Deserialize;
use hyper::rt::Future;

pub mod matches;
pub mod team;

const BASE_URL: String = String::from("https://www.thebluealliance.com/api/v3");

pub struct TBA {
    auth_key: &'static str,
}

impl TBA {
    pub fn new(auth_key: &'static str) -> TBA {
        TBA {
            auth_key,
        }
    }

    fn get<T>(&self, url: &str) -> impl Future<Item = Response<T>, Error = ()> {
        let uri = (BASE_URL + url).parse().expect("Cannot parse url.");

        let https = HttpsConnector::new(4).expect("Cannot create HttpsConnector.");

        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut request: Request<hyper::Body> = Request::builder()
            .uri(uri)
            .header("X-TBA-Auth-Key", self.auth_key)
            .body(hyper::Body::empty()).expect("Failed to construct request.");
        client.request(request)
            .and_then(|r| {
                let (parts, mut body) = r.into_parts();
                let mut data = bytes::Bytes::new();
                while let Ok(futures::Async::Ready(d)) = body.poll_data() {
                    data.into_bytes()
                }
                Response::from_parts(parts, body)
            })
            .and_then(|r| TBA::parseJson(r))
            .and_then(|r| r.expect("Failed to parse JSON."))
    }



    fn parseJson<T>(req: Response<Vec<u8>>) -> serde_json::Result<Response<T>>
        where for<'de> T: Deserialize<'de>,
    {
        let (parts, body) = req.into_parts();
        let body = serde_json::from_slice(&body)?;
        Ok(Response::from_parts(parts, body))
    }
}