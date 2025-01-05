use std::fmt::Debug;
use std::str::FromStr;

fn read_line_vec<T: FromStr<Err = E>, E: Debug>() -> Vec<T> {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<T> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| s.parse::<T>().expect(""))
        .collect();

    values
}

fn find_max_value<T: PartialOrd + Copy>(numbers: &[T]) -> Option<T> {
    numbers.iter().copied().reduce(|a, b| if a > b { a } else { b })
}

pub fn main() {
    let values: Vec<i32> = read_line_vec();
    let max = find_max_value(&values).unwrap();

    println!("{} eh o maior", max);
}
