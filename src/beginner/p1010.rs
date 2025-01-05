use std::io::{self, BufRead};

pub fn main() {
    let mut sum: f64 = 0_f64;
    let mut counter: u32 = 0;
    for line in io::stdin().lock().lines() {
        let values: Vec<f64> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<f64>().unwrap())
            .collect();

        sum += values[1] * values[2];

        counter += 1;
        if counter == 2 {
            break;
        }
    }

    println!("VALOR A PAGAR: R$ {:.2}", sum);
}
