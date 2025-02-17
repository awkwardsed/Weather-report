use reqwest;
use serde::{Deserialize, Serialize};

// Define a struct to hold the API response
#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Current {
    last_updated_epoch: i64,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i64,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: i64,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: i64,
    cloud: i64,
    feelslike_c: f64,
    feelslike_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Condition {
    text: String,
    icon: String,
    code: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "YOUR_API_KEY_HERE_PLEASE";
    let location = "London";

    let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}", api_key, location);

    let response = reqwest::get(&url).await?;

    let json: String = response.text().await?;
    let weather_response: WeatherResponse = serde_json::from_str(&json)?;

    println!("Location: {}", weather_response.location.name);
    println!("Temperature: {}°C", weather_response.current.temp_c);
    println!("Condition: {}", weather_response.current.condition.text);
    println!("Wind Speed (mph): {}", weather_response.current.wind_mph);
    println!("Wind Speed (kph): {}", weather_response.current.wind_kph);
    println!("Wind Direction: {}", weather_response.current.wind_dir);
    println!("Wind Degree: {}", weather_response.current.wind_degree);
    println!("Gust Speed (mph): {}", weather_response.current.gust_mph);
    println!("Gust Speed (kph): {}", weather_response.current.gust_kph);

    Ok(())
}
