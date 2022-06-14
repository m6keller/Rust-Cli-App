use structopt::StructOpt; // automatically parses the arguments
use exitfailure::{ExitFailure}; // error handling

mod lib {
    pub mod types;
}

use crate::lib::types::{Forecast, CLI};

#[tokio::main] // async main function
async fn main() -> Result<(), ExitFailure> {
    let args = CLI::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    println!("City: {} \nCountry Code: {} \nWeather: {}", args.city, args.country_code, response.weather.details.main);
    Ok(())
}
