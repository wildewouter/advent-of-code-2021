use std::str::FromStr;
use regex::Regex;
use crate::read;

pub fn run() {
    let content = read::file("input/day-4/input-example");
    let card = content.parse::<Bingo>();
}

struct Bingo {
    cards: Vec<BingoCard>
}

struct BingoCard {
    rows: Vec<BingoRow>,
}

struct BingoRow {
    cells: Vec<BingoCell>,
}

struct BingoCell {
    value: usize,
    hit: bool,
}

impl FromStr for Bingo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?sm)(((\d{1,2} {0,3})+\n){5})").unwrap();
        re.captures_iter(s).for_each(|mats| {
            dbg!(mats);
        });
        Ok(Bingo {
            cards: Vec::new(),
        })
    }
}