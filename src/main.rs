mod grid;
mod triangle;
mod triangle_number;
use grid::grid;
use std::io;
use triangle::trainage;
use triangle_number::triangle_number;
fn main() {
    println!("Enter the number to print your pattern");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a valid number");
    grid(n);
    trainage(n);
    triangle_number(n);
}
