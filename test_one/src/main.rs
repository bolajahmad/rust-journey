fn main() {
    println!("Fibonacci Printer");

    let test_value = 13;
    let tester = fibonacci(test_value);
    println!("Fibonacci of {test_value} is {tester}");
}

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181
fn fibonacci(value: i64) -> i64 {
    if value == 0 {
        return 0;
    } else if value == 1 {
        return 1;
    } else {
        fibonacci(value - 1) + fibonacci(value - 2)
    }
}
