// pattern in rust
/*
 * * * *
 * * * *
 * * * *
 * * * *
 *
 */

pub fn grid(n: i32) {
    for _ in 0..n {
        for _ in 0..n {
            print!("* ");
        }
        println!();
    }
}
