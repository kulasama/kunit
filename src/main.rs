extern crate clap;

use std::io;
use clap::{Arg, App, SubCommand};

fn kernel() -> Result<String, io::Error> {
    App::new("kunit")
        .version("1.0")
        .about("kunit container")
        .author("kula")
        .get_matches();

    let mut s = String::new();
    Ok(s)
}

fn main() {
    kernel();
    println!("kunit start");
}

