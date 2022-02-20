use std::cmp::PartialOrd;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use wordle::*;

struct Chmin<T: PartialOrd> {
    current: Option<T>,
}

impl<T: PartialOrd> Chmin<T> {
    fn new() -> Chmin<T> {
        Chmin { current: None }
    }

    fn apply(&mut self, val: T) -> bool {
        if let Some(current) = &self.current {
            if val <= *current {
                self.current = Some(val);
                true
            } else {
                false
            }
        } else {
            self.current = Some(val);
            true
        }
    }
}

fn main() -> io::Result<()> {
    let file_guess = File::open("../data/words_oa.txt")?;
    let reader_guess = BufReader::new(file_guess);
    let guesses = reader_guess
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();

    let file_answer = File::open("../data/words_ma.txt")?;
    let reader_answer = BufReader::new(file_answer);
    let answers = reader_answer
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();

    let mut chmin = Chmin::new();

    for guess in guesses.iter() {
        let mut counts = [0; (3 as usize).pow(5)];

        for answer in answers.iter().by_ref() {
            let hint = answer.get_hint(&guess).to_integer();
            counts[hint as usize] += 1;
        }

        let max = counts.into_iter().max().unwrap();
        if chmin.apply(max) {
            println!("{}: {}", guess, max);
        }
    }

    Ok(())
}
