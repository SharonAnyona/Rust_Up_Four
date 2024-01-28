//Basic function and variables in Rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 5; //immutable variable
    let mut y = 6; //mutable variable
    let z = add(x, y);
    let x = 2; //shadowing

    println!("The value of z is: {}", z);
}