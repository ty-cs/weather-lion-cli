use weather_lion::get_2hr_weather;

fn main() -> Result<(), ureq::Error> {
    get_2hr_weather()?;

    Ok(())
}
