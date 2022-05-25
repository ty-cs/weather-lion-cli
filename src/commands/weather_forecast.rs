use colored::{ColoredString, Colorize};
use std::time::{Duration, Instant};

use serde::Deserialize;

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
pub struct TwoHourWeatherInfo {
    area_metadata: Vec<AreaMetadata>,
    items: Vec<WeatherItem>,
    api_info: ApiInfo,
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
    } else if weather.contains("fair") {
        if weather.contains("night") {
            "☁️"
        } else {
            "🌤️"
        }
    } else if weather.contains("clear") {
        if weather.contains("night") {
            "🌙"
        } else {
            "☀️"
        }
    } else if weather.contains("rain") || weather.contains("showers") {
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
    let _started = Instant::now();
    // let pb = get_pb();

    // pb.set_message("Loading...");
    let body: TwoHourWeatherInfo =
        ureq::get("https://api.data.gov.sg/v1/environment/2-hour-weather-forecast")
            .timeout(Duration::new(3, 0)) // max 10 seconds
            .call()?
            .into_json()?;

    // pb.finish_and_clear();

    body.items.iter().for_each(|item| {
        println!("{}\n", "For the next 2 hour:".green());

        item.forecasts.iter().for_each(|forecast| {
            println!(
                // since emoji actually took 2 characters, so we need to put two spaces between 2nd and 3rd element
                "{:>24} {}  {}",
                forecast.area,
                get_emoji_from_weather_str(&forecast.forecast),
                forecast.forecast,
            );
        });
        println!(
            "\n{} {}",
            "Updated at".white(),
            item.update_timestamp.white()
        );

        // println!("\nUpdated at: {}", item.update_timestamp);
    });
    // TODO: only verbose output enabled
    // println!("\nDone in {:?}", started.elapsed());

    Ok(())
}

#[derive(Deserialize)]
struct Wind {
    speed: BetweenValue,
    direction: String,
}

#[derive(Deserialize)]
struct BetweenValue {
    low: i32,
    high: i32,
}

#[derive(Deserialize)]
struct General {
    forecast: String,
    relative_humidity: BetweenValue,
    temperature: BetweenValue,
    wind: Wind,
}

#[derive(Deserialize)]
struct OneDayWeatherInfo {
    items: Vec<OneDayWeatherInfoItem>,
    api_info: ApiInfo,
}

#[derive(Deserialize)]
struct OneDayWeatherInfoItem {
    update_timestamp: String,
    general: General,
}

fn get_styled_temp_str(temp: i32) -> ColoredString {
    if temp > 30 {
        return temp.to_string().red();
    }
    if temp < 20 {
        return temp.to_string().blue();
    }
    temp.to_string().normal()
}

pub fn get_24hr_weather() -> Result<(), ureq::Error> {
    let _started = Instant::now();
    // let pb = get_pb();

    // pb.set_message("Loading...");
    let body: OneDayWeatherInfo =
        ureq::get("https://api.data.gov.sg/v1/environment/24-hour-weather-forecast")
            .timeout(Duration::new(3, 0)) // max 10 seconds
            .call()?
            .into_json()?;

    // pb.finish_and_clear();

    body.items.iter().for_each(|item| {
        let general = &item.general;
        let General {
            forecast,
            relative_humidity,
            wind,
            temperature,
        } = general;

        println!("{}\n", "For the next 24 hour:".green());
        println!(
            "{:>11}: {:4}{}",
            "Forecast",
            get_emoji_from_weather_str(forecast),
            forecast
        );
        println!(
            "{:>11}: {:2} - {:2}",
            "Humidity", relative_humidity.low, relative_humidity.high
        );
        println!(
            "{:>11}: {:2} - {:2}",
            "Temperature",
            get_styled_temp_str(temperature.low),
            get_styled_temp_str(temperature.high)
        );
        println!(
            "{:>11}: {:2} - {:2}",
            "Wind speed", wind.speed.low, wind.speed.high
        );
        println!(
            "\n{} {}",
            "Updated at".white(),
            item.update_timestamp.white()
        );
    });
    // TODO: only verbose output enabled
    // println!("\nDone in {:?}", started.elapsed());

    Ok(())
}
