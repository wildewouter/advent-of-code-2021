use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content = read::file(&path);

    println!("Day four");
    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

fn part_one(content: &str) -> usize {
    let (hits_string, cards_string) = content.split_once("\n\n").unwrap();

    let hits: Vec<usize> = hits_string
        .split(',')
        .map(|hit_string| hit_string.parse::<usize>().unwrap())
        .collect();

    let cards: Vec<BingoCard> = cards_string
        .split("\n\n")
        .map(|card_string| card_string.parse::<BingoCard>().unwrap())
        .collect();

    let winners = play(cards, &hits);
    let (winning_card, final_number) = winners.first().unwrap();
    count_not_hit(winning_card) * final_number
}

fn part_two(content: &str) -> usize {
    let (hits_string, cards_string) = content.split_once("\n\n").unwrap();

    let hits: Vec<usize> = hits_string
        .split(',')
        .map(|hit_string| hit_string.parse::<usize>().unwrap())
        .collect();

    let cards: Vec<BingoCard> = cards_string
        .split("\n\n")
        .map(|card_string| card_string.parse::<BingoCard>().unwrap())
        .collect();

    let winners = play(cards, &hits);
    let (last_card, final_number) = winners.last().unwrap();
    count_not_hit(last_card) * final_number
}

fn play(mut cards: Vec<BingoCard>, hits: &[usize]) -> Vec<(BingoCard, usize)> {
    let mut winners = Vec::new();

    for hit in hits {
        if winners.len() == cards.len() {
            break;
        }

        cards.iter_mut().for_each(|card| {
            if card.is_complete {
                return;
            }

            card.hits.insert(*hit);

            let mut rows: HashMap<usize, usize> = HashMap::new();
            let mut columns: HashMap<usize, usize> = HashMap::new();

            card.hits.iter().for_each(|val| {
                if let Some((row, column)) = card.grid.get(val) {
                    rows.insert(*row, *rows.get(row).unwrap_or(&0) + 1);
                    columns.insert(*column, *columns.get(column).unwrap_or(&0) + 1);
                }
            });

            if rows.values().max().unwrap_or(&0) == &card.size
                || columns.values().max().unwrap_or(&0) == &card.size
            {
                card.is_complete = true;
                winners.push((card.clone(), *hit));
            }
        });
    }

    winners
}

fn count_not_hit(card: &BingoCard) -> usize {
    card.grid
        .keys()
        .filter(|num| !card.hits.contains(num))
        .sum()
}

#[derive(Debug, Clone)]
struct BingoCard {
    grid: HashMap<usize, (usize, usize)>,
    size: usize,
    hits: HashSet<usize>,
    is_complete: bool,
}

impl FromStr for BingoCard {
    type Err = ();

    fn from_str(card_string: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();

        for (y, row) in card_string.split('\n').enumerate() {
            for (x, cell) in row.split_whitespace().enumerate() {
                grid.insert(cell.parse::<usize>().unwrap(), (x, y));
            }
        }

        Ok(BingoCard {
            grid,
            size: 5,
            hits: HashSet::new(),
            is_complete: false,
        })
    }
}
