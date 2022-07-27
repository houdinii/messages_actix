#[macro_use]
extern crate actix_web;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port}
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
            .bind(("127.0.0.1", self.port))?
            .workers(8)
            .run()
    }
}
/// We are going to look for a particular header in a get request and respond with a message based
/// on the value of that header.
/// If the header is not present we will respond with a default message. Let’s define the
/// structure of this response
#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}