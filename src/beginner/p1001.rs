use std::io;

pub fn main() {
    let mut inputA = String::new();
    let mut inputB = String::new();

    io::stdin().read_line(&mut inputA).unwrap();
    io::stdin().read_line(&mut inputB).unwrap();

    let a: i32 = inputA.trim().parse().unwrap();
    let b: i32 = inputB.trim().parse().unwrap();

    println!("X = {}", a + b);
}
