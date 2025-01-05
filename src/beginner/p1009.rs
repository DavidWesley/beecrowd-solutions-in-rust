use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let salary: f64 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let quantity_sales: f64 = input.trim().parse().unwrap();

    println!("TOTAL = R$ {:.2}", salary + 0.15 * quantity_sales);
}
