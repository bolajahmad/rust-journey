use regex::Regex;
use clap::{Command, Arg};

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::new("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();
    
        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();

        let quote = "Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books. What do wee through millions of pages?";
        
        for line in quote.lines() {
            match re.find(line) {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
}
