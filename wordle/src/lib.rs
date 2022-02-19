#[derive(Debug, PartialEq)]
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

pub trait ToTileString {
    fn to_tile_string(&self) -> String;
}

impl ToTileString for Vec<Tile> {
    fn to_tile_string(&self) -> String {
        self.iter()
            .map(|tile| tile.to_string())
            .fold(String::from(""), |acc, s| acc + &s)
    }
}

pub fn hint(guess: &str, answer: &str) -> Vec<Tile> {
    guess
        .chars()
        .zip(answer.chars())
        .map(|(x, y)| {
            if x == y {
                Tile::Green
            } else if answer.contains(x) {
                Tile::Yellow
            } else {
                Tile::Black
            }
        })
        .collect()
}
