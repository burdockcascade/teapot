use std::collections::HashMap;

use http_body_util::Full;
use hyper::{Method, Response, Uri};
use hyper::body::Bytes;
use hyper::header::SERVER;

use crate::server::{HyperRequest, HyperResponse};

const SERVER_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct HttpServerRequest {
    pub method: Method,
    pub uri: Uri,
    pub headers: HashMap<String, String>,
}

impl From<&HyperRequest> for HttpServerRequest {
    fn from(req: &HyperRequest) -> Self {
        let mut headers = HashMap::new();
        for (key, value) in req.headers() {
            headers.insert(key.as_str().to_string(), value.to_str().unwrap().to_string());
        }
        HttpServerRequest {
            method: req.method().clone(),
            uri: req.uri().clone(),
            headers,
        }
    }
}

pub struct HttpServerResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Into<HyperResponse> for HttpServerResponse {
    fn into(self) -> HyperResponse {
        let mut res = Response::builder()
            .status(self.status)
            .header(SERVER, SERVER_NAME);
        for (key, value) in self.headers {
            res = res.header(key, value);
        }
        res.body(Full::new(Bytes::from(self.body))).unwrap()
    }
}