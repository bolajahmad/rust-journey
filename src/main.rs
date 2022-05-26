use regex::Regex;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let file = File::open("Cargo.toml").unwrap();
    let buf_reader = BufReader::new(file);

    for line_ in buf_reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some (_) => println!("{}", line),
            None => (),
        }
    }
}