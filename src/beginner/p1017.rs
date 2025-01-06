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

fn calculate_consumed_gas(time: u32, velocity: u32) -> f64 {
    (time * velocity) as f64 / 12.0
}

pub fn main() {
    let time: u32 = read_line_numeric().unwrap();
    let velocity: u32 = read_line_numeric().unwrap();

    println!("{:.3}", calculate_consumed_gas(time, velocity));
}
