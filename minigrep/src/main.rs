/// bring in dependency crates
use std::{env, fs};

fn main() {
    // fetch the arguments passed to the command line when cargo run is called
    let args: Vec<String> = env::args().collect();

    let (_query, file_path) = parse_config(&args);

    // read content of the file_path
    let file_content = fs::read_to_string(file_path).expect("Should be able to read the file");
    println!("With text: \n{file_content}");

    // dbg!(args);
}

fn parse_config(args: &Vec<String>) -> (&String, &String) {
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file, {file_path}");
    (query, file_path)
}
