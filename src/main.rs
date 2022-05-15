// use num::complex::Complex;

fn main() {
    let search_term = "jangilo";

    let quote = "\
    Every man shall die, and it's no joke
    dark square is a picture with no frames
    Its looks like a jangilova";

    // let mut line_num: usize = 1;

    // for line in quote.lines() {
    //     if line.contains(search_term) {
    //         println!("{}, line: {}", line, line_num);
    //     }
    //     line_num += 1;
    // }

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{}, line: {}", line, i + 1);
        }
    }
}
