//! [actix]: https://actix.rs/docs/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::env;

// #[get("/")]
#[get("/hello")]
async fn greet() -> impl Responder {
    // let to = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body("Hello world!")
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new().service(greet)
        // .route("/", |r| r.f(greet))
        // .route("/{name}", |r| r.f(greet))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server
            .bind(("0.0.0.0", port))
            .expect("Can not bind to port 8000")
    };

    server.run().await
}
