use std::io;
use std::num::ParseFloatError;

fn read_line_f64() -> Result<f64, ParseFloatError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse::<f64>();
}

fn calculate_sphere_volume(radius: f64) -> f64 {
    return (4.0_f64 / 3.0_f64) * 3.14159 * radius.powi(3);
}

pub fn main() {
    let radius = read_line_f64().unwrap();
    println!("VOLUME = {:.3}", calculate_sphere_volume(radius));
}
