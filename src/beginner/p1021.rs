use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;
use std::{io, vec};

use definitions::{NoteDetails, NoteInfo, NoteType};

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

mod definitions {
    #[derive(Debug)]
    pub enum NoteType {
        Note,
        Coin,
    }

    #[derive(Debug)]
    pub struct NoteInfo {
        pub note: f64,
        pub quantity: u32,
        pub note_type: NoteType,
    }

    #[derive(Debug)]
    pub struct NoteDetails {
        pub info: Vec<NoteInfo>,
    }
}

fn get_fewest_notes_sequence(cash: f64, banknotes: Option<&Vec<f64>>) -> NoteDetails {
    let default_banknotes: Vec<f64> = vec![
        200.0, 100.0, 50.0, 25.0, 10.0, 5.0, 2.0, 1.0, 0.5, 0.25, 0.1, 0.05, 0.01,
    ];

    let mut remaining: f64 = (cash * 100.0).round();

    let mut sorted_banknotes: Vec<f64> = banknotes
        .unwrap_or(&default_banknotes)
        .iter()
        .map(|note| (*note * 100.0).round())
        .collect();

    sorted_banknotes.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let info: Vec<NoteInfo> = sorted_banknotes
        .into_iter()
        .map(|note| {
            let quantity = (remaining / note).floor() as u32;
            remaining = remaining - (quantity as f64) * note;

            NoteInfo {
                note: (note as f64) / 100.0,
                quantity: quantity as u32,
                note_type: if note > 100.0 { NoteType::Note } else { NoteType::Coin },
            }
        })
        .collect();

    NoteDetails { info: info }
}

pub fn main() {
    let cash: f64 = read_line_numeric().unwrap();
    let current_banknotes = vec![100.0, 50.0, 20.0, 10.0, 5.0, 2.0, 1.0, 0.50, 0.25, 0.10, 0.05, 0.01];
    let details = get_fewest_notes_sequence(cash, Some(&current_banknotes));

    println!("NOTAS:");
    for info_note in details.info.iter() {
        match info_note.note_type {
            NoteType::Note => println!(
                "{qtd} nota(s) de R$ {note:.2}",
                qtd = info_note.quantity,
                note = info_note.note
            ),
            _ => continue,
        }
    }

    println!("MOEDAS:");
    for info_note in details.info.iter() {
        match info_note.note_type {
            NoteType::Coin => println!(
                "{qtd} moeda(s) de R$ {note:.2}",
                qtd = info_note.quantity,
                note = info_note.note
            ),
            _ => continue,
        }
    }
}
