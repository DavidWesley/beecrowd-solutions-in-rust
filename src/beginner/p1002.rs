use std::io;

fn calculate_circle_area(radius: f64) -> f64 {
    return radius.powi(2) * 3.14159;
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let radius: f64 = input.trim().parse().unwrap();

    println!("A={:.4}", calculate_circle_area(radius));
}
