use chrono::prelude::*;

use dotenv::dotenv;
use std::env;

fn main() {

    let utc = Utc::now();
    println!("\n{utc}\n");
    let ist = Local::now();
    println!("\n{ist}\n");
    
    dotenv().ok();
    let var = env::var("ADDRESS");

    match var {
        Ok(str) => println!("\n{str}\n"),
        Err(_e) => println!("\nError: {_e}\n")
    }
}
