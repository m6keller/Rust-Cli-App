use structopt::StructOpt; // automatically parses the arguments
use exitfailure::{ExitFailure}; // error handling

mod lib {
    pub mod types;
}

struct FunctionMap {
    get_weather: any
}

fn print_type_of<T>(_: &T) {

    println!("{}", std::any::type_name::<T>());

}

use crate::lib::types::{Forecast, CLI};

#[tokio::main] // async main function
async fn main() -> Result<(), ExitFailure> {
    let function_map ={ FunctionMap {
        get_weather: firstCliApp::get_weather
    }};
    print_type_of(&get_weather);
    println!("{}", funtion_map["get_weather"]);
    return get_weather().await;
    // return function_map["get_weather"] 
    // let args = CLI::from_args();
    // let response = Forecast::get(&args.city, &args.country_code).await?;

    // println!("City: {} \nCountry Code: {} \nWeather: {}", args.city, args.country_code, response.weather.details.main);
    // Ok(())
}

async fn get_weather() -> Result<(), ExitFailure> {
    let args = CLI::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    println!("City: {} \nCountry Code: {} \nWeather: {}", args.city, args.country_code, response.weather.details.main);
    Ok(())

}
