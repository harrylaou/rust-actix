use crate::request_models::time_series::TimeSeries;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::RwLock;

pub async fn greet(req: HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");

    HttpResponse::Ok().body(format!("Hello {}!", to))
}

pub fn time_series_handler(time_series_state: web::Data<RwLock<TimeSeries>>) -> HttpResponse {
    let time_series_guard = time_series_state.read().unwrap();
    let time_series: &TimeSeries = &*time_series_guard;
    HttpResponse::Ok().json(time_series)
    // match  time_series_result  {
    //     Ok(time_series)=>     HttpResponse::Ok().json(time_series),
    //     Err(err)=> {
    //         eprintln!("Problem time_series: {}", err);
    //
    //         err.as_response_error().error_response()
    //         // HttpResponse::InternalServerError().body(err)
    //     }
    // }
}
