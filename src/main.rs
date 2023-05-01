fn main() {
    let mut x: i32 = 21;
    let mut count: i32 = 0;
    while x >= 0 {
        count += 1;
        x -= 3;
    }
    println!("{}", count);
}
