use std::fmt;
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
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hours, self.minutes, self.seconds)
    }
}

fn calculate_duration_from_seconds(secs: u32) -> Duration {
    const FACTORS: [u32; 3] = [1, 60_u32.pow(1), 60_u32.pow(2)];
    let mut values: [u32; 3] = [0, 0, 0];
    let mut remaining_secs = secs.clone();

    for i in (0..3).rev() {
        let factor = FACTORS[i];
        values[i] = remaining_secs / factor;
        remaining_secs -= values[i] * factor;
    }

    Duration {
        seconds: values[0],
        minutes: values[1],
        hours: values[2],
    }
}

pub fn main() {
    let time: u32 = read_line_numeric().unwrap();

    println!("{}", calculate_duration_from_seconds(time))
}
