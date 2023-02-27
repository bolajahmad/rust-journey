/// bring in dependency crates
use std::{env, fs};

fn main() {
    // fetch the arguments passed to the command line when cargo run is called
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // read content of the file_path
    let file_content =
        fs::read_to_string(&config.file_path).expect("Should be able to read the file");
    println!("With text: \n{file_content}");

    // dbg!(args);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough command line arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        println!("Searching for {query}");
        println!("In file, {file_path}");
        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }
}
