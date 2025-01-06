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
    const CAR_X: f64 = 60.0;
    const CAR_Y: f64 = 90.0;

    let space: f64 = read_line_numeric::<u32>().unwrap() as f64;
    let time_in_minutes: f64 = (space * 60_f64 / (CAR_Y - CAR_X)).trunc();

    println!("{} minutos", time_in_minutes);
}
