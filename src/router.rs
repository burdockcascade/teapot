use std::sync::{Arc, Mutex};

use hyper::Method;

use crate::message::{HttpServerRequest, HttpServerResponse};
use crate::server::HyperRequest;

#[derive(Clone)]
pub enum Route {
    Http {
        path: String,
        method: Method,
        handler: Arc<Mutex<dyn RouteHandler>>,
    },
}

pub trait RouteHandler: Send + Sync + 'static {
    fn on_request(&mut self, request: &HttpServerRequest) -> HttpServerResponse;
}

#[derive(Clone)]
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn add_route(mut self, route: Route) {
        self.routes.push(route);
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: vec![],
        }
    }
}

impl Router {
    
    pub fn find_route(&self, request: &HyperRequest) -> Option<HttpServerResponse> {
        for route in &self.routes {
            match route {
                Route::Http { path, method, handler } => {
                    if request.uri().path() == path && request.method() == *method {

                        let mut handler = handler.lock().unwrap();

                        let server_request: HttpServerRequest = HttpServerRequest::from(request);
                        
                        return Some(handler.on_request(&server_request));
                    }
                }
            }
        }
        
        None
    }
    
    
}

pub struct RouteBuilder;

impl RouteBuilder {
    pub fn http() -> HttpRouteBuilder {
        HttpRouteBuilder::new()
    }
}

pub struct HttpRouteBuilder {
    path: Option<String>,
    method: Option<Method>,
    handler: Option<Arc<Mutex<dyn RouteHandler>>>
}

impl HttpRouteBuilder {
    fn new() -> Self {
        HttpRouteBuilder {
            path: None,
            method: None,
            handler: None
        }
    }

    pub fn get(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self.method = Some(Method::GET);
        self
    }

    pub fn post(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self.method = Some(Method::POST);
        self
    }

    pub fn put(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self.method = Some(Method::PUT);
        self
    }

    pub fn delete(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self.method = Some(Method::DELETE);
        self
    }

    pub fn handler(mut self, controller: Arc<Mutex<impl RouteHandler + 'static>>) -> Self {
        self.handler = Some(controller.clone());
        self
    }

    pub fn build(self) -> Result<Route, String> {
        if self.path.is_none() {
            return Err(String::from("Path is required"));
        }

        if self.method.is_none() {
            return Err(String::from("Method is required"));
        }

        if self.handler.is_none() {
            return Err(String::from("Handler is required"));
        }

        Ok(Route::Http {
            path: self.path.unwrap(),
            method: self.method.unwrap(),
            handler: self.handler.unwrap()
        })
    }
}