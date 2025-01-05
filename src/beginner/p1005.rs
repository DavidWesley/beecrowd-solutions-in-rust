use std::io;

pub fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();

    let a: f64 = input_a.trim().parse().unwrap();
    let b: f64 = input_b.trim().parse().unwrap();

    let average: f64 = (a * 3.5 + b * 7.5) / 11_f64;

    println!("MEDIA = {:.5}", average);
}
