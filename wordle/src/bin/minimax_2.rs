use itertools::Itertools;
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

const M: i32 = 243;

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

    let mut memo = vec![vec![0; guesses.len()]; answers.len()];

    for (i, answer) in answers.iter().enumerate() {
        for (j, guess) in guesses.iter().enumerate() {
            memo[i][j] = answer.get_hint(&guess).to_integer();
        }
    }

    for (i, j) in (0..guesses.len()).cartesian_product(0..guesses.len()) {
        if i % 100 == 0 && j == 0 {
            println!("---{}: {}/{}---", guesses[i], i, guesses.len());
        }
        if i >= j {
            continue;
        }

        let mut counts = [0; (M * M) as usize];

        //for answer in answers.iter().by_ref() {
        for k in 0..answers.len() {
            let i = memo[k][i] * M + memo[k][j];
            counts[i as usize] += 1;
        }

        let max = counts.into_iter().max().unwrap();
        if chmin.apply(max) {
            println!("{}, {}: {}", guesses[i], guesses[j], max);
        }
    }

    Ok(())
}
