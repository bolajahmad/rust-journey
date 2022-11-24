fn main() {
    // Defines an Octal number
    let octal_digit: i16 = 0o_236;
    const USER_PROFILE: (&str, u8, &str) = ("James", 23, "M");
    // println!("Spaces, \"{spaces}\"");
    // let spaces = spaces.len();
    let (name, age, sex) = USER_PROFILE;

    println!("My name is {name} and I am {age}, also a {sex}");
}
