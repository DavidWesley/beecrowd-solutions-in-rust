use std::fmt::Debug;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn distance_between(p1: &Point, p2: &Point) -> f64 {
        (p2.x - p1.x).hypot(p2.y - p1.y)
    }
}

fn parse_input<T: FromStr>(input: &str) -> Result<T, T::Err> {
    input.trim().parse::<T>()
}

fn read_line_vec<T: FromStr<Err = E>, E: Debug>() -> Vec<T> {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<T> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| parse_input::<T>(&s).unwrap())
        .collect();

    values
}

pub fn main() {
    let input_a: Vec<f64> = read_line_vec();
    let input_b: Vec<f64> = read_line_vec();

    let point_a = Point {
        x: input_a[0],
        y: input_a[1],
    };

    let point_b = Point {
        x: input_b[0],
        y: input_b[1],
    };

    println!("{:.4}", Point::distance_between(&point_a, &point_b));
}
