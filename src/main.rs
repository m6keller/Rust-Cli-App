use structopt::StructOpt; // automatically parses the arguments
use exitfailure::{ExitFailure}; // error handling
use serde_derive::{Deserialize, Serialize}; // deserialize the json file
use reqwest::Url; // url parsing   

// use openweathermap.org/api

const WEATHER_API_KEY: &str = "c990fe48e97da433565f1f9e5bd1c560";

#[derive(StructOpt)] // handles/parses CLI arguments
struct CLI {
    city: String,
    country_code: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f32,
    lat: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f32,
    feels_like: f32,
    pressure: i32,
    humidity: i32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: i32,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32
}

impl Forecast {
    async fn get( city: &String, country_code: &String ) -> Result<Self, ExitFailure> {
        let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", city, country_code, WEATHER_API_KEY);
        let url: Url = Url::parse(&*url)?; // ? propogarses the error

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;// ? propogarses the error

        Ok(resp)

    }
}

#[tokio::main] // async main function
async fn main() -> Result<(), ExitFailure> {
    let args = CLI::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    println!("City: {} \nCountry Code: {} \nWeather: {}", args.city, args.country_code, response.weather.details.main);
    Ok(())
}

// #[tokio::main] // async main function
// async fn main() -> Result<(), ExitFailure> {
//     let args = CLI::from_args();
//     // let response = Forecast::get(&args.city, &args.country_code).await?;
//     let req: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", args.city, args.country_code, WEATHER_API_KEY);
//     let url: Url = Url::parse(&*req)?;
//     let response = reqwest::get(url).await?;

//     // println!("City: {} \nCountry Code: {} \nWeather: {}", args.city, args.country_code, response.weather.main);
//     println!("{}", response.text().await?);
//     Ok(())
// }



