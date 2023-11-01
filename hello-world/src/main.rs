// use std::io;

fn main() {
    // Variables
    let x = 0; // this is immutable
    println!("Hello World!");
    println!("{}", x);
    println!("{x}");

    let mut y = 1;
    println!("{y}");
    y = 2;
    println!("{y}")
}
