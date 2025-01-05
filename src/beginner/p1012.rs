use std::convert::TryInto;

mod area {
    pub fn calculate_rect_area(x: f64, y: f64) -> f64 {
        x * y
    }

    pub fn calculate_square_area(l: f64) -> f64 {
        calculate_rect_area(l, l)
    }

    pub fn calculate_triangle_area(base: f64, height: f64) -> f64 {
        (base * height) / 2.0
    }

    pub fn calculate_trapeze_area(base_a: f64, base_b: f64, height: f64) -> f64 {
        ((base_b + base_a) * height) / 2.0
    }

    pub fn calculate_circle_area(radius: f64) -> f64 {
        3.14159 * radius.powi(2)
    }
}

fn read_line_f64_vec() -> Vec<f64> {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<f64> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    return values;
}

pub fn main() {
    let [a, b, c]: [f64; 3] = read_line_f64_vec().try_into().unwrap();

    println!("TRIANGULO: {:.3}", area::calculate_triangle_area(a, c));
    println!("CIRCULO: {:.3}", area::calculate_circle_area(c));
    println!("TRAPEZIO: {:.3}", area::calculate_trapeze_area(a, b, c));
    println!("QUADRADO: {:.3}", area::calculate_square_area(b));
    println!("RETANGULO: {:.3}", area::calculate_rect_area(a, b));
}
