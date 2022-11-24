fn main() {
    let x = 200;
    const HOUR_TO_SECONDS: u32 = 1 * 60 * 60;

    // x = 102;
    // println!("The number is: {}", x);

    {
        let x = 10;
        println!("The number is shadowed as {}", x)
    }
    println!("The number is: {x}");
}
