extern crate reqwest;

use std::env;

fn main() -> Result<(), reqwest::Error> {
    let mut args = env::args();
    let _ = args.next();

    let start = args.next().unwrap();
    let end = args.next().unwrap();
    let api_key = args.next().unwrap();

    let url = format!("https://maps.googleapis.com/maps/api/directions/json?origin={}&destination={}&mode=driving&departure_time=now&key={}", start, end, api_key);
    let echo_json: serde_json::Value = reqwest::Client::new()
        .get(&url)
        .send()?
        .json()?;

    println!("{:#?}", echo_json);
    Ok(())
}
