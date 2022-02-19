use wordle::*;
use rstest::*;

#[rstest]
#[case("arise", "knoll", vec![Tile::Black, Tile::Black, Tile::Black, Tile::Black, Tile::Black])]
#[case("angel", "knoll", vec![Tile::Black, Tile::Green, Tile::Black, Tile::Black, Tile::Green])]
#[case("could", "knoll", vec![Tile::Black, Tile::Yellow, Tile::Black, Tile::Green, Tile::Black])]
#[case("lemon", "knoll", vec![Tile::Yellow, Tile::Black, Tile::Black, Tile::Yellow, Tile::Yellow])]
#[case("knoll", "knoll", vec![Tile::Green, Tile::Green, Tile::Green, Tile::Green, Tile::Green])]
#[case("olive", "price", vec![Tile::Black, Tile::Black, Tile::Green, Tile::Black, Tile::Green])]
#[case("proof", "snake", vec![Tile::Black, Tile::Black, Tile::Black, Tile::Black, Tile::Black])]
fn hint_works(#[case] input: &str, #[case] answer: &str, #[case] expected: Vec<Tile>){
    assert_eq!(hint(input, answer), expected);
}

//🟩
//🟨
//⬛
#[rstest]
#[case(vec![Tile::Green, Tile::Yellow, Tile::Black], "🟩🟨⬛")]
#[case(vec![Tile::Green, Tile::Yellow, Tile::Black, Tile::Green, Tile::Yellow], "🟩🟨⬛🟩🟨")]
fn to_tile_string_works(#[case] vec: Vec<Tile>, #[case] tiles: &str){
    assert_eq!(vec.to_tile_string(), tiles)
}
