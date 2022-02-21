use std::fs::File;
use std::io::Write;
use std::io::{self, prelude::*, BufReader};

pub mod cmp;

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Green,
    Yellow,
    Black,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        let block = match self {
            Tile::Green => "🟩",
            Tile::Yellow => "🟨",
            Tile::Black => "⬛",
        };
        String::from(block)
    }
}

impl Tile {
    pub fn to_integer(&self) -> i32 {
        match self {
            Tile::Green => 2,
            Tile::Yellow => 1,
            Tile::Black => 0,
        }
    }

    pub fn from_integer(num: i32) -> Tile {
        match num {
            0 => Tile::Black,
            1 => Tile::Yellow,
            2 => Tile::Green,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Hint {
    pub tiles: Vec<Tile>,
}

impl Hint {
    pub fn to_integer(&self) -> i32 {
        self.tiles.iter().fold(0, |acc, x| acc * 3 + x.to_integer())
    }

    pub fn from_integer(num: i32, size: usize) -> Hint {
        let mut num = num;
        let mut tiles = vec![Tile::Black; size];
        for i in (0..size).rev() {
            let res = num % 3;
            tiles[i] = Tile::from_integer(res);
            num = (num - res) / 3;
        }
        Hint { tiles }
    }
}

impl ToString for Hint {
    fn to_string(&self) -> String {
        self.tiles
            .iter()
            .map(|tile| tile.to_string())
            .fold(String::from(""), |acc, s| acc + &s)
    }
}

pub trait GetHint {
    fn get_hint(&self, guess: &str) -> Hint;
}

impl GetHint for String {
    fn get_hint(&self, guess: &str) -> Hint {
        let tiles = guess
            .chars()
            .zip(self.chars())
            .map(|(x, y)| {
                if x == y {
                    Tile::Green
                } else if self.contains(x) {
                    Tile::Yellow
                } else {
                    Tile::Black
                }
            })
            .collect();
        Hint { tiles }
    }
}

pub enum WordType {
    All,
    Answer,
}

pub fn get_words(w: WordType) -> Vec<String> {
    let path = match w {
        WordType::All => "../data/words_oa.txt",
        WordType::Answer => "../data/words_ma.txt",
    };
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>()
}
