fn main() {
    let x = 2;
    let y= 4;
    let w = mul(x,y);
    
    println!("The multiplication of {} and {} is: {}", x,y,w);
}
fn mul(x: i32 ,y: i32) -> i32{
    x * y
}