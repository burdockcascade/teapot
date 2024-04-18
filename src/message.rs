use std::collections::HashMap;

use http_body_util::Full;
use hyper::{Method, Response, StatusCode, Uri};
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
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<Body>,
}

impl HttpServerResponse {
    pub fn builder() -> HttpServerResponseBuilder {
        HttpServerResponseBuilder::new()
    }
}

impl Into<HyperResponse> for HttpServerResponse {
    fn into(self) -> HyperResponse {
        
        // Create response
        let mut res = Response::builder()
            .status(self.status)
            .header(SERVER, SERVER_NAME);
        
        // Add headers
        for (key, value) in self.headers {
            res = res.header(key, value);
        }
        
        // Add body
        match self.body {
            Some(body) => {
                res = res.header("Content-Type", body.content_type);
                res.body(Full::new(body.content)).unwrap()
            },
            None => res.body(Full::new(Bytes::new())).unwrap()
        }
    }
}

pub struct Body {
    content_type: String,
    content: Bytes,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            content_type: "text/plain".to_string(),
            content: Bytes::new(),
        }
    }
}

impl Body {
    pub fn new(content_type: &str, content: Vec<u8>) -> Self {
        Body {
            content_type: content_type.to_string(),
            content: Bytes::from(content),
        }
    }

    pub fn html<V>(content: V) -> Self where V: Into<Vec<u8>> {
        Body::new("text/html", content.into())
    }

    pub fn json<V>(content: V) -> Self where V: Into<Vec<u8>> {
        Body::new("application/json", content.into())
    }
    
    pub fn text<V>(content: V) -> Self where V: Into<Vec<u8>> {
        Body::new("text/plain", content.into())
    }
}

pub struct HttpServerResponseBuilder {
    status: Option<StatusCode>,
    headers: HashMap<String, String>,
    body: Option<Body>,
}

impl HttpServerResponseBuilder {
    pub fn new() -> Self {
        HttpServerResponseBuilder {
            status: Some(StatusCode::OK),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> HttpServerResponse {
        HttpServerResponse {
            status: self.status.unwrap_or(StatusCode::OK),
            headers: self.headers,
            body: self.body,
        }
    }
}