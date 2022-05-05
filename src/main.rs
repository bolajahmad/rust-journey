use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: 5.0 };
    let b = Complex::new(1.1, 2.2);
    
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
