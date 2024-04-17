use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

use teapot::message::{HttpServerRequest, HttpServerResponse};
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
    
    let my_teapot = Arc::new(Mutex::new(MyHandler { counter: 0 }));
    
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

struct MyHandler {
    counter: u16
}

impl RouteHandler for MyHandler {
    fn on_request(&mut self, request: &HttpServerRequest) -> HttpServerResponse {
        
        self.counter += 1;
        let message = format!("Serving cup number {}", self.counter);
        
        HttpServerResponse {
            status: 200,
            headers: HashMap::new(),
            body: message.into_bytes(),
        }
    }
}
