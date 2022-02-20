use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use wordle::*;

fn main() -> io::Result<()> {
    let file = File::open("../data/words_ma.txt")?;
    let reader = BufReader::new(file);

    let answer = "knoll";

    for line in reader.lines().take(100) {
        let line = line?;
        let guess = line.trim();

        println!("{} {:?}", guess, answer.to_string().get_hint(guess));
    }

    Ok(())
}
