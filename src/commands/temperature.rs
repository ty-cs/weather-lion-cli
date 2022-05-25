use colored::Colorize;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct TemperatureInfo {
    items: Vec<TemperatureItem>,
}

#[derive(Deserialize)]
struct ReadingInfo {
    station_id: String,
    value: f32,
}

#[derive(Deserialize)]
struct TemperatureItem {
    timestamp: String,
    readings: Vec<ReadingInfo>,
}

pub fn get_temperature() -> anyhow::Result<()> {
    let body: TemperatureInfo = ureq::get("https://api.data.gov.sg/v1/environment/air-temperature")
        .timeout(Duration::new(3, 0)) // max 10 seconds
        .call()?
        .into_json()?;
    let items = body.items;

    items.iter().for_each(|item| {
        println!("{}\n", "Real time Air Temperature:".green());
        let readings = &item.readings;
        readings.iter().for_each(|reading| {
            let ReadingInfo { station_id, value } = reading;
            println!("{:>4}: {:.1}", station_id, value);
        });
        println!("\n{} {}", "Updated at".white(), item.timestamp.white());
    });

    Ok(())
}
