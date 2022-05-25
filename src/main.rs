use weather_lion::commands::weather_forecast::get_24hr_weather;

fn main() -> Result<(), ureq::Error> {
    get_24hr_weather()?;
    Ok(())
}
