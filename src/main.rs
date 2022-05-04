fn main() {
    let a = 20;
    let b: i32 = 30;
    
    let addition = a + b;
    println!("{} + {} = {}", a, b, addition);

    let d: i64 = 1_000_000;
    println!("{}", d.pow(2));
    println!("{} + {} = {}", a, d, (a + d));

    let e =  [
        5.0,
        5f32,
        5.0_f32
    ];
    println!("{:3}", e[1]);
}
