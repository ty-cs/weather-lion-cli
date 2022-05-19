use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::time::{Duration, Instant};

#[derive(Deserialize)]
struct Forecast {
    area: String,
    forecast: String,
}

#[derive(Deserialize)]
struct Location {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize)]
struct AreaMetadata {
    // location name
    name: String,
    label_location: Location,
}

#[derive(Deserialize)]
struct ValidPeriod {
    start: String,
    end: String,
}

#[derive(Deserialize)]
struct WeatherItem {
    update_timestamp: String,
    valid_period: ValidPeriod,
    timestamp: String,
    forecasts: Vec<Forecast>,
}

#[derive(Deserialize)]
struct ApiInfo {
    status: String,
}

impl std::fmt::Display for ApiInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.status)
    }
}

#[derive(Deserialize)]
pub struct WeatherInfo {
    area_metadata: Vec<AreaMetadata>,
    items: Vec<WeatherItem>,
    api_info: ApiInfo,
}

fn get_pb() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb
}

fn get_emoji_from_weather(weather: &str) -> &str {
    match weather {
        "Clear" => "☀️",
        "Clouds" => "☁️",
        "Partly Cloudy" => "🌤️",
        "Partly Cloudy (Night)" => "☁️",
        "Drizzle" => "🌧",
        "Rain" => "🌧",
        "Thunderstorm" => "⛈",
        "Snow" => "❄️",
        "Mist" => "🌁",
        _ => " ",
    }
}

fn get_emoji_from_weather_str(weather: &str) -> &str {
    let weather = weather.to_lowercase();
    return if weather.contains("cloud") {
        if weather.contains("partly") {
            if weather.contains("night") {
                "☁️"
            } else {
                "🌤️"
            }
        } else {
            "☁️"
        }
    } else if weather.contains("clear") {
        if weather.contains("night") {
            "🌙"
        } else {
            "☀️"
        }
    } else if weather.contains("rain") {
        if weather.contains("light") {
            "🌦"
        } else {
            "🌧"
        }
    } else if weather.contains("snow") {
        "❄️"
    } else if weather.contains("thunder") {
        "⛈"
    } else if weather.contains("mist") {
        "🌁"
    } else {
        " "
    };
}

pub fn get_2hr_weather() -> Result<(), ureq::Error> {
    let started = Instant::now();
    // let pb = get_pb();

    // pb.set_message("Loading...");
    let body: WeatherInfo =
        ureq::get("https://api.data.gov.sg/v1/environment/2-hour-weather-forecast")
            .timeout(Duration::new(3, 0)) // max 10 seconds
            .call()?
            .into_json()?;

    // pb.finish_and_clear();

    body.items.iter().for_each(|item| {
        item.forecasts.iter().for_each(|forecast| {
            println!(
                "{:24}{} {emoji}",
                forecast.area,
                forecast.forecast,
                emoji = get_emoji_from_weather_str(&forecast.forecast)
            );
        });
        println!("\nUpdated at: {}", item.update_timestamp);
    });
    // TODO: only verbose output enabled
    // println!("\nDone in {:?}", started.elapsed());

    Ok(())
}
