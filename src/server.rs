use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use http_body_util::Full;
use hyper::{body, Request, Response};
use hyper::body::Bytes;
use hyper::header::SERVER;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper_util::rt::TokioIo;
use log::{debug, info};
use tokio::net::TcpListener;

use crate::router::{Route, Router};

pub type IncomingBody = body::Incoming;
pub type HyperRequest = Request<IncomingBody>;
pub type HyperResponse = Response<Full<Bytes>>;

pub struct HttpServer {
    address: String,
    port: u16,
    svc: TeapotService,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            address: "127.0.0.1".to_string(),
            port: 80,
            svc: TeapotService::default(),
        }
    }

    pub fn builder() -> HttpServerBuilder {
        HttpServerBuilder::default()
    }

    pub async fn start(self) -> Result<(), Box<dyn Error + Send + Sync>> {

        let address = format!("{}:{}", self.address, self.port);

        info!("Starting server on {}", address);

        // We create a TcpListener and bind it to 127.0.0.1:3000
        let listener = TcpListener::bind(address).await?;

        loop {
            let (stream, _address) = listener.accept().await?;
            let io = TokioIo::new(stream);

            let svc_clone = self.svc.clone();

            tokio::task::spawn(async {
                if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }
}

#[derive(Clone)]
struct TeapotService {
    router: Router,
}

impl Default for TeapotService {
    fn default() -> Self {
        TeapotService {
            router: Router::new(),
        }
    }
}

impl Service<Request<IncomingBody>> for TeapotService {

    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {

        debug!("Incoming request: {:?} for {:?}", req.method(), req.uri());

        let res = match self.router.find_route(&req) {
            Some(server_response) => server_response.into(),
            None => Response::builder()
                .status(404)
                .header(SERVER, "teapot/0.1")
                .body(Full::new(Bytes::from("Route not found"))).unwrap()
        };

        Box::pin(async { Ok(res) })
    }
}

pub struct HttpServerBuilder {
    address: String,
    port: u16,
    routes: Vec<Route>,
}

impl Default for HttpServerBuilder {
    fn default() -> Self {
        HttpServerBuilder {
            address: String::from("127.0.0.1"),
            port: 80,
            routes: Vec::new(),
        }
    }
}

impl HttpServerBuilder {

    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    pub fn address(&mut self, address: &str) -> &mut Self {
        self.address = address.to_string();
        self
    }

    pub fn route(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn build(&self) -> HttpServer {
        HttpServer {
            address: self.address.clone(),
            port: self.port,
            svc: TeapotService {
                router: Router {
                    routes: self.routes.clone(),
                }
            }
        }
    }

    pub fn start(&self) {
        let server = self.build();
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
            .block_on(server.start())
            .unwrap();
    }

}