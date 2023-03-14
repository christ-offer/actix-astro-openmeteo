use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct OpenMeteoRequest {
    city: String,
}

struct DailyData {
    temperature_2m_max: f32,
    temperature_2m_min: f32,
    precipitation_sum: f32,
}

#[derive(Serialize)]
struct OpenMeteoResponse {
    date: DailyData,
}


#[get("/api/openmeteo")]
async fn openmeteo(open_meteo_request: web::Json<OpenMeteoRequest>) -> impl Responder {
    // The app should accept lat and lon as a JSON object
    // and return a JSON object with the following fields:
    // {
    // date: {
    //          temperature_2m_max: json.tenperature_2m_max, 
    //          temperature_2m_min: json.temperature_2m_min, 
    //          precipitation_sum: json.precipitation_sum
    //      }
    //}
    


    HttpResponse::Ok().body("Hello world!")
}
