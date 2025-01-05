use std::io;

pub fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();

    let a: f64 = input_a.trim().parse().unwrap();
    let b: f64 = input_b.trim().parse().unwrap();
    let c: f64 = input_c.trim().parse().unwrap();

    let average: f64 = ((a * 2.0) + (3.0 * b) + (5.0 * c)) / 10_f64;

    println!("MEDIA = {:.1}", average);
}
