use rstest::*;
use wordle::*;

#[rstest]
#[case("arise", "knoll", vec![Tile::Black, Tile::Black, Tile::Black, Tile::Black, Tile::Black])]
#[case("angel", "knoll", vec![Tile::Black, Tile::Green, Tile::Black, Tile::Black, Tile::Green])]
#[case("could", "knoll", vec![Tile::Black, Tile::Yellow, Tile::Black, Tile::Green, Tile::Black])]
#[case("lemon", "knoll", vec![Tile::Yellow, Tile::Black, Tile::Black, Tile::Yellow, Tile::Yellow])]
#[case("knoll", "knoll", vec![Tile::Green, Tile::Green, Tile::Green, Tile::Green, Tile::Green])]
#[case("olive", "price", vec![Tile::Black, Tile::Black, Tile::Green, Tile::Black, Tile::Green])]
#[case("proof", "snake", vec![Tile::Black, Tile::Black, Tile::Black, Tile::Black, Tile::Black])]
fn hint_works(#[case] input: &str, #[case] answer: &str, #[case] expected: Vec<Tile>) {
    let hint = answer.to_string().get_hint(input);
    assert_eq!(hint, Hint { tiles: expected });
}

//🟩
//🟨
//⬛
#[rstest]
#[case(vec![Tile::Green, Tile::Yellow, Tile::Black], "🟩🟨⬛")]
#[case(vec![Tile::Green, Tile::Yellow, Tile::Black, Tile::Green, Tile::Yellow], "🟩🟨⬛🟩🟨")]
fn to_tile_string_works(#[case] tiles: Vec<Tile>, #[case] s: &str) {
    let hint = Hint { tiles };
    assert_eq!(hint.to_string(), s)
}

#[rstest]
#[case(vec![Tile::Green, Tile::Green, Tile::Green, Tile::Green, Tile::Green], 242)]
#[case(vec![Tile::Black, Tile::Black, Tile::Black, Tile::Black, Tile::Green], 2)]
#[case(vec![Tile::Black, Tile::Black, Tile::Black, Tile::Black, Tile::Yellow], 1)]
fn hint_to_integer_works(#[case] tiles: Vec<Tile>, #[case] num: i32) {
    let hint = Hint { tiles };
    assert_eq!(hint.to_integer(), num)
}

#[rstest]
#[case(242, vec![Tile::Green, Tile::Green, Tile::Green, Tile::Green, Tile::Green])]
#[case(236, vec![Tile::Green, Tile::Green, Tile::Green, Tile::Black, Tile::Green])]
#[case(8, vec![Tile::Green, Tile::Green])]
#[case(7, vec![Tile::Green, Tile::Yellow])]
#[case(1, vec![Tile::Black, Tile::Yellow])]
fn hint_from_integer_works(#[case] num: i32, #[case] tiles: Vec<Tile>) {
    let hint = Hint::from_integer(num, tiles.len());
    assert_eq!(hint, Hint { tiles })
}
