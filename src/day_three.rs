use std::borrow::Borrow;
use std::process::exit;
use std::str::FromStr;
use regex::{Match, Regex};
use crate::read;

pub fn run() {
    let content: Vec<Binary> = read::file("input/day-3/input-example").lines().filter_map(|line| line.parse::<Binary>().ok()).collect();

    println!("Day three:");
    // println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

struct Binary {
    split: Vec<usize>,
}

impl FromStr for Binary {
    type Err = ();

    fn from_str(input: &str) -> Result<Binary, ()> {
        Ok(Binary {
            split: input.split("").into_iter().filter_map(|val| val.parse::<usize>().ok()).collect(),
        })
    }
}

impl ToString for Binary {
    fn to_string(&self) -> String {
        let mut result: String = String::new();

        for item in self.split.iter() {
            result.push_str(item.to_string().as_str());
        }

        result
    }
}

impl Into<usize> for Binary {
    fn into(self) -> usize {
        usize::from_str_radix(self.to_string().as_str(), 2).unwrap()
    }
}

fn part_one(input: &[Binary]) -> usize {
    let gamma: usize = get_most_common(input).into();
    let epsilon: usize = get_least_common(input).into();

    gamma * epsilon
}

fn count(input: &[Binary]) -> Vec<usize> {
    let start: Option<Vec<usize>> = None;

    input.iter().fold(start, |total, next| {
        match total {
            None => Some(next.split.clone()),
            Some(result) => {
                let mut counter: Vec<usize> = result.clone();
                for (index, bit) in next.split.iter().enumerate() {
                    counter[index] = result[index] + bit;
                }

                Some(counter)
            }
        }
    }).unwrap()
}

fn get_most_common(input: &[Binary]) -> Binary {
    let count = count(input);

    Binary {
        split: count
            .iter().map(|&bit| {
            if bit > input.iter().count() / 2 {
                return 1;
            }

            0
        }).collect()
    }
}

fn get_least_common(input: &[Binary]) -> Binary {
    let count = count(input);

    Binary {
        split: count
            .iter().map(|&bit| {
            if bit <= input.iter().count() / 2 {
                return 1;
            }

            0
        }).collect()
    }
}

fn part_two(input: &[Binary]) -> usize {
    let most_common = get_most_common(input);
    let least_common = get_least_common(input);

    let oxigen: Vec<&Binary> = input.iter().filter(|binary| {
        binary.split.iter().enumerate().fold(true, |result, (index, bit)| {
            if !result {
                return result;
            }

            &most_common.split[index] == bit
        })
    }).collect();

    let co2 = least_common.split.iter().enumerate().fold(input, |result, (index, bit)| {
        if result.len() == 1 {
            return result;
        }

        result.into_iter().filter(|binary| {
            binary.split[index] == *bit
        }).collect()
    });
    // let co2: Vec<&Binary> = input.iter().filter(|binary| {
    //     binary.split.iter().enumerate().fold(true, |result, (index, bit)| {
    //         if !result {
    //             return result;
    //         }
    //
    //         &least_common.split[index] == bit
    //     })
    // }).collect();

    //todo: Get items from array and product
    dbg!(co2.len());
    0
}