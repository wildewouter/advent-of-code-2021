use crate::read;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn run() {
    let content = read::file("input/day-4/input");
    let (hits_string, cards_string) = content.split_once("\n\n").unwrap();

    println!("Part one: {}", part_one(hits_string, cards_string));
}

fn part_one(hits_string: &str, cards_string: &str) -> usize {
    let hits: Vec<usize> = hits_string
        .split(",")
        .map(|hit_string| hit_string.parse::<usize>().unwrap())
        .collect();

    let cards: Vec<BingoCard> = cards_string
        .split("\n\n")
        .map(|card_string| card_string.parse::<BingoCard>().unwrap())
        .collect();

    let (winning_card, final_number) = play(cards, &hits).unwrap();

    count_not_hit(&winning_card) * final_number
}

fn play(mut cards: Vec<BingoCard>, hits: &[usize]) -> Option<(BingoCard, usize)> {
    for hit in hits {
        cards.iter_mut().for_each(|card| {
            card.hits.insert(*hit);
        });

        let result: Option<BingoCard> = cards.iter().fold(None, |result, card| match result {
            Some(winner) => Some(winner),
            None => {
                let mut rows: HashMap<usize, usize> = HashMap::new();
                let mut columns: HashMap<usize, usize> = HashMap::new();

                card.hits.iter().for_each(|val| match card.grid.get(&val) {
                    Some((row, column)) => {
                        rows.insert(*row, *rows.get(row).unwrap_or(&0) + 1);
                        columns.insert(*column, *columns.get(column).unwrap_or(&0) + 1);
                    }
                    None => (),
                });

                let is_win = rows.values().max().unwrap_or(&0) == &card.size
                    || columns.values().max().unwrap_or(&0) == &card.size;

                if !is_win {
                    return None;
                }

                Some(BingoCard {
                    grid: card.grid.clone(),
                    size: card.size.clone(),
                    hits: card.hits.clone(),
                })
            }
        });

        if result.is_some() {
            return Some((result.unwrap(), *hit));
        }
    }

    None
}

fn count_not_hit(card: &BingoCard) -> usize {
    card.grid
        .keys()
        .filter(|num| !card.hits.contains(num))
        .sum()
}

#[derive(Debug)]
struct BingoCard {
    grid: HashMap<usize, (usize, usize)>,
    size: usize,
    hits: HashSet<usize>,
}

impl FromStr for BingoCard {
    type Err = ();

    fn from_str(card_string: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();

        for (y, row) in card_string.split("\n").enumerate() {
            for (x, cell) in row.split_whitespace().enumerate() {
                grid.insert(cell.parse::<usize>().unwrap(), (x, y));
            }
        }

        Ok(BingoCard {
            grid,
            size: 5,
            hits: HashSet::new(),
        })
    }
}
