pub mod worldometers {
    use crate::request_models::time_series::*;
    use actix_web::client::{Client, ClientBuilder};
    use actix_web::Result as ActixResult;
    use serde_json::Result as JsonResult;
    use std::time::Duration;

    pub async fn get_time_series() -> ActixResult<TimeSeries> {
        let client: Client = ClientBuilder::new()
            .timeout(Duration::from_secs(35))
            .finish();
        let response = client
            .get("https://covid2019-api.herokuapp.com/v2/timeseries/confirmed")
            .header("User-Agent", "Actix-web")
            .send()
            .await;
        let body_bytes = response?.body().limit(1_000_000).await?;
        let body_str = std::str::from_utf8(body_bytes.as_ref())?;
        let time_series: JsonResult<TimeSeries> = serde_json::from_str(body_str);
        time_series.map_err(|err| err.into())
    }
}
