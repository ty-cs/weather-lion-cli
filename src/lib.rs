use std::time::{Duration, Instant};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Forecast {
    area: String,
    forecast: String,
}

#[derive(Deserialize, Debug)]
struct Location {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
struct AreaMetadata {
    // location name
    name: String,
    label_location: Location,
}

#[derive(Deserialize, Debug)]
struct ValidPeriod {
    start: String,
    end: String,
}

#[derive(Deserialize, Debug)]
struct WeatherItem {
    update_timestamp: String,
    valid_period: ValidPeriod,
    timestamp: String,
    forecasts: Vec<Forecast>,
}

#[derive(Deserialize, Debug)]
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
        ProgressStyle::with_template("{spinner} {msg}  {elapsed_precise}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    );
    pb
}
pub fn get_2hr_weather() -> Result<(), ureq::Error> {
    let started = Instant::now();
    let pb = get_pb();
    pb.set_message("Loading...");
    let body: WeatherInfo = ureq::get("https://api.data.gov.sg/v1/environment/2-hour-weather-forecast")
        .call()?
        .into_json()?;

    pb.finish_and_clear();
    // body.area_metadata.iter().for_each(|area| {
    //     println!("{}", area.name);
    // });
    body.items.iter().for_each(|item| {
        item.forecasts.iter().for_each(|forecast| {
            println!("{:24} => {}",forecast.area, forecast.forecast);
        });
        println!("Updated at: {}", item.update_timestamp);
    });
    println!("\nDone in {:?}", started.elapsed());

    Ok(())
}
