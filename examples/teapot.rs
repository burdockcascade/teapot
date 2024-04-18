
use std::sync::{Arc, Mutex};
use hyper::StatusCode;

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

use teapot::message::{Body, HttpServerRequest, HttpServerResponse};
use teapot::router::{RouteBuilder, RouteHandler};
use teapot::server::HttpServer;

fn main() {

    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    );
    
    info!("Starting teapot server example...");
    
    let my_teapot = Arc::new(Mutex::new(TeaMaker { counter: 0 }));
    
    HttpServer::builder()
        .address("127.0.0.1")
        .port(80)
        .route(RouteBuilder::http()
            .get("/teapot")
            .handler(my_teapot.clone())
            .build()
            .expect("valid route"))
        .start();

}

struct TeaMaker {
    counter: u16
}

impl RouteHandler for TeaMaker {
    fn on_request(&mut self, _request: &HttpServerRequest) -> HttpServerResponse {
        
        self.counter += 1;
        let message = format!("Serving cup number {}", self.counter);
        
        HttpServerResponse::builder()
            .status(StatusCode::IM_A_TEAPOT)
            .body(Body::text(message))
            .build()
    }
}
