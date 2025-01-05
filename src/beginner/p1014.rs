use std::io;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

trait LikeNumber {}
impl<T> LikeNumber for T where
    T: Add<Output = Self>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>
        + Rem<Output = Self>
        + Copy
        + PartialEq
        + PartialOrd
{
}

fn parse_input<T: FromStr>(input: &str) -> Result<T, T::Err> {
    input.trim().parse::<T>()
}

fn read_line_numeric<T: LikeNumber + FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    parse_input::<T>(&input)
}

pub fn main() {
    let distance: u32 = read_line_numeric().unwrap();
    let consumed_gas: f64 = read_line_numeric().unwrap();

    println!("{:.3} km/l", (distance as f64) / consumed_gas);
}
