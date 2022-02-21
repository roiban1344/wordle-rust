use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Write};
use wordle::cmp::*;
use wordle::*;

fn max_entropy(depth: u32, group: &Vec<String>, guesses: &Vec<String>, mut out: &File) {
    let mut chmax = Chmax::new();
    let n = group.len();

    let mut guess_opt = String::new();
    let mut subgroups_opt = BTreeMap::<i32, Vec<String>>::new();
    for guess in guesses {
        let mut subgroups = BTreeMap::<i32, Vec<String>>::new(); //u32でいい
        for answer in group {
            let hint = answer.get_hint(guess);
            let k = hint.to_integer();
            if let Some(v) = subgroups.get_mut(&k) {
                v.push(answer.to_string());
            } else {
                subgroups.insert(k, vec![answer.to_string()]);
            }
        }
        let entropy = subgroups
            .iter()
            .filter_map(|(_, v)| {
                if v.len() == 0 {
                    None
                } else {
                    let p = (v.len() as f64) / (n as f64);
                    Some(-p * p.ln())
                }
            })
            .sum::<f64>();
        if chmax.apply(entropy) {
            guess_opt = guess.to_string();
            subgroups_opt = subgroups.clone();
        }
    }

    let mut subgroups_opt = subgroups_opt.iter().collect::<Vec<_>>();
    subgroups_opt
        .sort_by(|(_, subgroup_0), (_, subgroup_1)| subgroup_0.len().cmp(&subgroup_1.len()));

    writeln!(out, "{}[{}]", "\t".repeat(depth as usize), guess_opt);
    for (&k, subgroup) in subgroups_opt {
        let hint = Hint::from_integer(k, 5);
        if subgroup.len() == 1 {
            writeln!(
                out,
                "{}{}→{}",
                "\t".repeat(depth as usize),
                hint.to_string(),
                subgroup[0]
            );
        } else {
            writeln!(out, "{}{}", "\t".repeat(depth as usize), hint.to_string());
            max_entropy(depth + 1, subgroup, guesses, out);
        }
    }
}

fn main() -> io::Result<()> {
    let guesses = get_words(WordType::All);
    let answers = get_words(WordType::Answer);

    let out = File::create("../data/cheat_sheet.txt")?;

    max_entropy(0, &answers, &guesses, &out);

    Ok(())
}
