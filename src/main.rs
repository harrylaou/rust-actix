use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
use std::env;
mod handlers;
mod request_models;
mod types;
mod web_client;
use handlers::*;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let mut listen_fd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            // .service(greet) //used with // #[get("/")]
            .route("/", web::get().to(greet))
            .route("/hello/{name}", web::get().to(greet))
            .route("/timeseries", web::get().to(time_series))
    });

    server = if let Some(l) = listen_fd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server
            .bind(("0.0.0.0", port))
            .unwrap_or_else(|_| panic!("Can not bind to port {}", port))
    };

    server.run().await
}
