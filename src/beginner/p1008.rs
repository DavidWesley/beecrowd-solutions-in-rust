// aconst input = require('fs').readFileSync('//dev/stdin', 'utf8')
// const [number, hours, valueByHours] = input.split('\n')
//     .map((value, index) => {
//         if (index == 2) return parseFloat(value).toFixed(2)
//         else return parseInt(value)
//     })
//     .filter((value) => !isNaN(value))

// function salary(h = 0, vH = 0) {
//     let sal = h * vH
//     return sal.toFixed(2)
// }

// console.log(`NUMBER = ${number}`)
// console.log(`SALARY = U$ ${salary(hours, valueByHours)}`)

use std::io;

pub fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();

    let number: u32 = input_a.trim().parse().unwrap();
    let hours: u32 = input_b.trim().parse().unwrap();
    let cost_per_hours: f64 = input_c.trim().parse().unwrap();

    println!("NUMBER = {}", number);
    println!("SALARY = U$ {:.2}", (hours as f64) * cost_per_hours);
}
