extern crate clap;

use std::io;
use clap::{Arg, App, SubCommand};

extern crate oci;
extern crate time;
extern crate file;

mod utils;






fn kunit() -> Result<String, io::Error> {
    utils::load_spec();
    let matches = App::new("kunit")
        .version("0.1")
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

    let s = String::new();
    Ok(s)
}



fn main() {
    kunit();
    println!("kunit start");
}

