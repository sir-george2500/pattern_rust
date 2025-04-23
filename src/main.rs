mod grid;
mod triangle;
use grid::grid;
use std::io;
use triangle::trainage;

fn main() {
    println!("Enter the number to print your pattern");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a valid number");
    grid(n);
    trainage(n);
}
