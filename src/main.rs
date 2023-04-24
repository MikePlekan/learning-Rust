use std::io;
use rust_decimal::prelude::*;

//https://code.visualstudio.com/docs/languages/rust
//https://doc.rust-lang.org/rust-by-example/
//https://youtu.be/uKlHwko36c4 make libary for many langs
//https://www.tutorialspoint.com/rust/index.html

fn main() -> io::Result<()> {
    let now = std::time::Instant::now();
    let pi = pi(10000000);
    let elapsed = now.elapsed();
    println!("pi is {} and it took {} seconds", pi, elapsed.as_secs_f64());
    Ok(())
}
//Gregory–Leibniz series
fn pi(n: u64) -> Decimal {
    let mut pi = Decimal::new(0, 0);
    let mut k = Decimal::new(0, 0);
    let one = Decimal::new(1, 0);
    let two = Decimal::new(2, 0);
    let four = Decimal::new(4, 0);
    let mut sign = one;
    for _ in 0..n {
        pi = pi + sign * (four / (two * k + one));
        k = k + one;
        sign = sign * -one;
    }
    pi
}