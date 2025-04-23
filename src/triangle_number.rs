pub fn triangle_number(n: i32) {
    for i in 0..n {
        for j in 0..i {
            print!("{} ", j);
        }
        println!();
    }
}
