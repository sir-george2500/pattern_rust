pub fn triangle_number(n: i32) {
    for i in 1..n {
        for j in 1..i {
            print!("{} ", j);
        }
        println!();
    }
}
