use std::io;

//https://code.visualstudio.com/docs/languages/rust
//https://doc.rust-lang.org/rust-by-example/
//https://youtu.be/uKlHwko36c4 make libary for many langs
//https://www.tutorialspoint.com/rust/index.htm
use io::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() -> io::Result<()> {
    /* let mut buffer = [0; 100];
    let mut f =io::stdin();
    let 
    //wait for input until enter is pressed
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!("The string: {}", String::from_utf8_lossy(&buffer[..n])); */

    let handles=vec![
        thread::spawn(||{
            for i in 1..10{
                println!("hi number {} from the spawned thread!",i);
                thread::sleep(Duration::from_millis(1));
            }
        }),
        thread::spawn(||{
            for i in 101..200{
                println!("hi number {} from the spawned thread!",i);
                thread::sleep(Duration::from_millis(1));
            }
        }),
    ];
    for handle in handles{
        handle.join().unwrap();
    }
    println!("{}",checkeven(4));
    Ok(())
}
fn checkeven(n: i32) -> String {
    if n & 1 == 0 {
        return String::from("True");
    } else {
        return String::from("False");
    }
}