use chrono::{DateTime, Local, NaiveDateTime, TimeZone}; // Importă și TimeZone
use reqwest;
use serde_json::Value;
use std::error::Error;


pub struct CityInfo {
    pub name: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub description: String,
    pub humidity: u8,
    pub wind_speed: f64,
    pub wind_deg: Option<u16>,
    pub pressure: u16,
    pub visibility: Option<u64>,
    pub sunrise: Option<DateTime<Local>>, 
    pub sunset: Option<DateTime<Local>>,  
    pub latitude: f64,
    pub longitude: f64,
    pub last_updated: DateTime<Local>,
}

pub async fn get_data(city: &str) -> Result<CityInfo, Box<dyn Error>> {
    let api_key = "09df7bcf5399f84a6a61229b61dbe517";
    let url_str = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    //MAKE THE FETCH
    let response = reqwest::get(&url_str).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get data for {}: {}", city, response.status()).into());
    }

    let response_json: Value = response.json().await?;

    let name = response_json["name"].as_str().unwrap_or("Unknown").to_string();
    let temperature = response_json["main"]["temp"].as_f64().unwrap_or(0.0);
    let feels_like = response_json["main"]["feels_like"].as_f64().unwrap_or(0.0);
    let description = response_json["weather"][0]["description"]
        .as_str()
        .unwrap_or("No description")
        .to_string();
    let humidity = response_json["main"]["humidity"].as_u64().unwrap_or(0) as u8;
    let wind_speed = response_json["wind"]["speed"].as_f64().unwrap_or(0.0);
    let wind_deg = response_json["wind"]["deg"].as_u64().map(|deg| deg as u16);
    let pressure = response_json["main"]["pressure"].as_u64().unwrap_or(0) as u16;
    let visibility = response_json["visibility"].as_u64();
    let latitude = response_json["coord"]["lat"].as_f64().unwrap_or(0.0);
    let longitude = response_json["coord"]["lon"].as_f64().unwrap_or(0.0);

    let sunrise = response_json["sys"]["sunrise"].as_i64().map(|timestamp| {
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        Local.from_utc_datetime(&naive)
    });

    let sunset = response_json["sys"]["sunset"].as_i64().map(|timestamp| {
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        Local.from_utc_datetime(&naive)
    });

    let city_info = CityInfo {
        name,
        temperature,
        feels_like,
        description,
        humidity,
        wind_speed,
        wind_deg,
        pressure,
        visibility,
        sunrise,
        sunset,
        latitude,
        longitude,
        last_updated: Local::now(),
    };

    Ok(city_info)
}