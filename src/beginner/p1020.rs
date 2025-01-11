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

#[derive(Debug, Default, Clone, Copy)]
struct Duration {
    days: u32,
    months: u32,
    years: u32,
}

fn calculate_duration_from_days(days: u32) -> Duration {
    const FACTORS: [u32; 3] = [1, 30, 365];
    let mut values: [u32; 3] = [0, 0, 0];
    let mut remaining_time = days.clone();

    for i in (0..3).rev() {
        let factor: u32 = FACTORS[i];
        values[i] = remaining_time / factor;
        remaining_time -= values[i] * factor;
    }

    Duration {
        days: values[0],
        months: values[1],
        years: values[2],
    }
}

pub fn main() {
    let days: u32 = read_line_numeric().unwrap();
    let duration = calculate_duration_from_days(days);

    println!(
        "{y} ano(s)\n{m} mes(es)\n{d} dia(s)",
        y = duration.years,
        m = duration.months,
        d = duration.days
    )
}
