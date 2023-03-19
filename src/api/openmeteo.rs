use actix_web::{post, web, HttpResponse, Responder, Result, error::ResponseError, http::StatusCode};
use reqwest::Client;
use std::fmt::{self, Display, Formatter};

// get a model from ../models/mod.rs
use crate::models::openmeteo::Daily;
use crate::models::openmeteo::MinMaxMean;
use crate::models::openmeteo::OpenMeteo;
use crate::models::openmeteo::OpenMeteoRequest;
use crate::models::openmeteo::ReturnOpenMeteo;
use crate::models::openmeteo::StatMap;
use crate::models::openmeteo::Stats;
use crate::models::osm::Direction;

const USER_AGENT: &str = "User-Agent";
const CONTENT_TYPE: &str = "Content-Type";
const ACCEPT: &str = "Accept";

// Customer error handling for reqwest so the ? traits work properly inside the openmeteo function
#[derive(Debug)]
pub enum CustomError {
    ReqwestError(reqwest::Error),
    // You can add more error types if needed
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ReqwestError(e) => write!(f, "ReqwestError: {}", e),
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::ReqwestError(_) => HttpResponse::new(StatusCode::BAD_GATEWAY),
        }
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError::ReqwestError(error)
    }
}




impl Daily {
    pub fn calculate_stats(&self) -> Stats {
        Stats {
            temperature_2m_max: Self::calculate_stat_map(&self.temperature_2m_max, &self.time),
            temperature_2m_min: Self::calculate_stat_map(&self.temperature_2m_min, &self.time),
            temperature_2m_mean: Self::calculate_stat_map(
                &self.temperature_2m_mean,
                &self.time,
            ),
            apparent_temperature_max: Self::calculate_stat_map(
                &self.apparent_temperature_max,
                &self.time,
            ),
            apparent_temperature_min: Self::calculate_stat_map(
                &self.apparent_temperature_min,
                &self.time,
            ),
            apparent_temperature_mean: Self::calculate_stat_map(
                &self.apparent_temperature_mean,
                &self.time,
            ),
            shortwave_radiation_sum: Self::calculate_stat_map(
                &self.shortwave_radiation_sum,
                &self.time,
            ),
            precipitation_sum: Self::calculate_stat_map(&self.precipitation_sum, &self.time),
            precipitation_hours: Self::calculate_stat_map(
                &self.precipitation_hours,
                &self.time,
            ),
            windspeed_10m_max: Self::calculate_stat_map(&self.windspeed_10m_max, &self.time),
            windgusts_10m_max: Self::calculate_stat_map(&self.windgusts_10m_max, &self.time),
            et0_fao_evapotranspiration: Self::calculate_stat_map(
                &self.et0_fao_evapotranspiration,
                &self.time,
            ),
            snowfall_sum: Self::calculate_stat_map(&self.snowfall_sum, &self.time),
        }
    }

    fn calculate_stat_map(data: &Vec<Option<f64>>, times: &Vec<Option<String>>) -> StatMap {
        let mut min = None;
        let mut max = None;
        let mut sum = 0.0;
        let mut count = 0;

        for (i, value) in data.iter().enumerate() {
            if let Some(val) = value {
                if let Some(_time) = times[i].as_ref() {
                    count += 1;
                    sum += val;

                    match min {
                        None => min = Some((i, *val)),
                        Some((_, min_val)) if *val < min_val => min = Some((i, *val)),
                        _ => (),
                    }

                    match max {
                        None => max = Some((i, *val)),
                        Some((_, max_val)) if *val > max_val => max = Some((i, *val)),
                        _ => (),
                    }
                }
            }
        }

        let mean = if count > 0 { sum / count as f64 } else { 0.0 };

        StatMap {
            min: MinMaxMean {
                time: min
                    .map(|(i, _)| times[i].clone().unwrap())
                    .unwrap_or("".to_string()),
                value: min.map(|(_, v)| v).unwrap_or(0.0),
            },
            max: MinMaxMean {
                time: max
                    .map(|(i, _)| times[i].clone().unwrap())
                    .unwrap_or("".to_string()),
                value: max.map(|(_, v)| v).unwrap_or(0.0),
            },
            mean: MinMaxMean {
                time: "".to_string(),
                value: mean,
            },
        }
    }
}



#[post("/api/openmeteo")]
async fn openmeteo(open_meteo_request: web::Json<OpenMeteoRequest>) -> Result<impl Responder, CustomError> {
    let city = &open_meteo_request.city;

    // Get the coordinates of the city from the OSM API
    let url: String = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json",
        city
    );
    println!("url: {}", url);
    let client = Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "reqwest")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<Vec<Direction>>()
        .await?;

    let osm_data = response;
    let lat = osm_data[0].lat.clone();
    let lon = osm_data[0].lon.clone();

    let rez = Direction { lat, lon };

    let start_date = &open_meteo_request.start_date;
    let end_date = &open_meteo_request.end_date;
    //let filter_request = &open_meteo_request.filter_request;

    // Get the weather data from the OpenMeteo API
    let meteo_url = format!("https://archive-api.open-meteo.com/v1/archive?latitude={}&longitude={}&start_date={}&end_date={}&daily=weathercode,temperature_2m_max,temperature_2m_min,temperature_2m_mean,apparent_temperature_max,apparent_temperature_min,apparent_temperature_mean,sunrise,sunset,shortwave_radiation_sum,precipitation_sum,rain_sum,snowfall_sum,precipitation_hours,windspeed_10m_max,windgusts_10m_max,winddirection_10m_dominant,et0_fao_evapotranspiration&timezone=Europe%2FBerlin", {rez.lat.clone()}, {rez.lon.clone()}, {start_date.clone()}, {end_date.clone()});
    let meteo_response = client
        .get(&meteo_url)
        .header(USER_AGENT, "reqwest")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<OpenMeteo>()
        .await?;

    let daily_data_copy = meteo_response.clone();
    let stats = daily_data_copy.daily.calculate_stats();
    let sorted_daily_data = daily_data_copy.daily.clone();


    let return_open_meteo = ReturnOpenMeteo {
        latitude: daily_data_copy.latitude,
        longitude: daily_data_copy.longitude,
        generationtime_ms: daily_data_copy.generationtime_ms,
        utc_offset_seconds: daily_data_copy.utc_offset_seconds,
        timezone: daily_data_copy.timezone,
        timezone_abbreviation: daily_data_copy.timezone_abbreviation,
        elevation: daily_data_copy.elevation,
        daily: daily_data_copy.daily,
        daily_units: daily_data_copy.daily_units,
        daily_sorted: sorted_daily_data,
        stats: stats,
    };

    Ok(HttpResponse::Ok().json(&return_open_meteo))
}
