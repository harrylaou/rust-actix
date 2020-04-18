use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
use std::env;

mod global;
mod handlers;
mod request_models;
mod web_client;

use crate::request_models::time_series::TimeSeries;
use actix_web::web::Data;
use handlers::*;
use job_scheduler::{Job, JobScheduler};
use std::sync::RwLock;
use web_client::worldometers::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.

    let time_series: TimeSeries = get_time_series().await.unwrap();

    let time_series_state: Data<RwLock<TimeSeries>> = web::Data::new(RwLock::new(time_series));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let mut listen_fd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(time_series_state.clone())
            // .service(greet) //used with // #[get("/")]
            .route("/", web::get().to(greet))
            .route("/hello/{name}", web::get().to(greet))
            .route("/timeseries", web::get().to(time_series_handler))
    });

    server = if let Some(l) = listen_fd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server
            .bind(("0.0.0.0", port))
            .unwrap_or_else(|_| panic!("Can not bind to port {}", port))
    };

    let mut job_scheduler = JobScheduler::new();

    job_scheduler.add(Job::new("0 * * * * *".parse().unwrap(), || {
        println!("I get executed every minute");
        async {
            let time_series_result = get_time_series().await;
            match time_series_result {
                Ok(time_series) => {
                    let time_series_state_in = time_series_state.clone();
                    let mut time_series_guard = time_series_state_in.write().unwrap();
                    *time_series_guard = time_series;
                }
                Err(e) => eprintln!("Problem getting new timeseries: {}", e),
            }
        };
    }));

    server.run().await
}
