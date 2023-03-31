use std::io;

//https://code.visualstudio.com/docs/languages/rust
//https://doc.rust-lang.org/rust-by-example/
use io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = [0; 100];
    let mut f =io::stdin();
    
    //wait for input until enter is pressed
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!("The string: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
