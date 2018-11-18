extern crate clap;

use std::io;
use clap::{Arg, App, SubCommand};





fn kernel() -> Result<String, io::Error> {
    let matches = App::new("kunit")
        .version("1.0")
        .about("kunit container")
        .author("kula")
        .subcommand(
            SubCommand::with_name("run")
                .about("Run a kunit container"),
        )
        .get_matches();
    match matches.subcommand() {
        ("run", Some(run_matches)) => {
           println!("run kunit")
        }
        _ => println!("command not recognized"),
    }

    let mut s = String::new();
    Ok(s)
}


fn main() {
    kernel();
    println!("kunit start");
}

