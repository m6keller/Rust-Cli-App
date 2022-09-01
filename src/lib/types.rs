

use structopt::StructOpt; // automatically parses the arguments
use serde_derive::{Deserialize, Serialize}; // deserialize the json file
use reqwest::Url; // url parsing   
use exitfailure::{ExitFailure}; // error handling

mod hidden;

#[derive(StructOpt)] // handles/parses CLI arguments
pub struct CLI {
    pub city: String,
    pub country_code: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Weather,
    pub base: String,
    pub main: Temps,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32
}


impl Forecast {
    pub async fn get( city: &String, country_code: &String ) -> Result<Self, ExitFailure> {
        let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", city, country_code, crate::lib::types::hidden::WEATHER_API_KEY);
        let url: Url = Url::parse(&*url)?; // ? propogates the error

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;// ? propogarses the error

        Ok(resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temps {
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: i32,
    pub humidity: i32,
    pub temp_min: f32,
    pub temp_max: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub r#type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32
}
