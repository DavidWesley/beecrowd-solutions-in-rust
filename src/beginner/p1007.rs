use std::io;

pub fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();
    let mut input_d = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();
    io::stdin().read_line(&mut input_d).unwrap();

    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();
    let c: i32 = input_c.trim().parse().unwrap();
    let d: i32 = input_d.trim().parse().unwrap();

    let difference = a * b - c * d;

    println!("DIFERENCA = {}", difference);
}
