use super::web_client::worldometers::*;
use actix_web::Result as ActixResult;
use actix_web::{HttpRequest, HttpResponse, Responder};
pub async fn greet(req: HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");

    HttpResponse::Ok().body(format!("Hello {}!", to))
}

pub async fn time_series() -> ActixResult<HttpResponse> {
    let time_series = get_time_series().await?;
    // Ok(HttpResponse::Ok().body(time_series.dt))
    Ok(HttpResponse::Ok().json(time_series))
}
