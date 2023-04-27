//https://code.visualstudio.com/docs/languages/rust
//https://doc.rust-lang.org/rust-by-example/
//https://youtu.be/uKlHwko36c4 make libary for many langs
//https://www.tutorialspoint.com/rust/index.html

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rust_decimal::Decimal;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/pi")]
fn pi_route() -> String {
    let now = std::time::Instant::now();
    let pi = pi(10000000);
    let elapsed = now.elapsed();
    format!("pi is {} and it took {} seconds", pi, elapsed.as_secs_f64())
}
fn main() {
    rocket::ignite().mount("/", routes![index, pi_route]).launch();
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
//function pi() {
//            let i = 1n;
//            let x = 3n * (10n ** 10020n);
//            let pi = x;
//            while (x > 0) {
//                x = x * i / ((i + 1n) * 4n);
//                pi += x / (i + 2n);
//                i += 2n;
//                document.body.innerText =pi;
//            }
//            return(pi / (10n ** 20n));
//        }
/* fn pi2()->Decimal{
    let mut pi = Decimal::new(0, 0);
    let mut i = Decimal::new(1, 0);
    let mut x = Decimal::new(3, 0) * (Decimal::new(10, 0).pow(10020));
    let one = Decimal::new(1, 0);
    let two = Decimal::new(2, 0);
    let four = Decimal::new(4, 0);
    let zero = Decimal::new(0, 0);
    while x > zero {
        x = x * i / ((i + one) * four);
        pi += x / (i + two);
        i += two;
    }
    pi / (Decimal::new(10, 0).pow(20))
} */