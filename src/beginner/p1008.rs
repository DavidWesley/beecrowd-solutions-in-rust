use std::io;

pub fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();

    let number: u32 = input_a.trim().parse().unwrap();
    let hours: u32 = input_b.trim().parse().unwrap();
    let cost_per_hours: f64 = input_c.trim().parse().unwrap();

    println!("NUMBER = {}", number);
    println!("SALARY = U$ {:.2}", (hours as f64) * cost_per_hours);
}
