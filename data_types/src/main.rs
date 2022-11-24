use std::io;

fn main() {
    const MONTHS_AVAILABLE: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    loop {
        let mut index = String::new();
        println!("Enter your birth month");
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index.trim().parse().expect("Invalid index received");

        if index >= 12 {
            continue;
        }

        println!("Your date is {}", MONTHS_AVAILABLE[index]);
        break;
    }
}
