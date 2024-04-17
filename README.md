# TEAPOT
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![GHA Build Status](https://github.com/burdockcascade/teapot/workflows/CI/badge.svg)](https://github.com/burdockcascade/teapot/actions?query=workflow%3ACI)

## Status
This project is in early development and is not yet ready for production use.

## Example
```rust
fn main() {
    
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
    fn on_request(&mut self, _request: &HttpServerRequest) -> HttpServerResponse {
        
        self.counter += 1;
        let message = format!("Serving cup number {}", self.counter);
        
        HttpServerResponse::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(message)
            .build()
    }
}
```

## License
Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)