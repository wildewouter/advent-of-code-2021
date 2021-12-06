use crate::read;
use std::str::FromStr;

pub fn run() {
    let content: Vec<Binary> = read::file("input/day-3/input")
        .lines()
        .filter_map(|line| line.parse::<Binary>().ok())
        .collect();

    println!("Day three:");
    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

#[derive(Clone, Debug)]
struct Binary {
    split: Vec<usize>,
}

impl FromStr for Binary {
    type Err = ();

    fn from_str(input: &str) -> Result<Binary, ()> {
        Ok(Binary {
            split: input
                .split("")
                .into_iter()
                .filter_map(|val| val.parse::<usize>().ok())
                .collect(),
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

    input
        .iter()
        .fold(start, |total, next| match total {
            None => Some(next.split.clone()),
            Some(result) => {
                let mut counter: Vec<usize> = result.clone();
                for (index, bit) in next.split.iter().enumerate() {
                    counter[index] = result[index] + bit;
                }
                Some(counter)
            }
        })
        .unwrap()
}

fn get_most_common(input: &[Binary]) -> Binary {
    let count = count(input);
    Binary {
        split: count
            .iter()
            .map(|&bit| {
                if bit * 2 >= input.len() {
                    return 1;
                }

                0
            })
            .collect(),
    }
}

fn get_least_common(input: &[Binary]) -> Binary {
    let count = count(input);

    Binary {
        split: count
            .into_iter()
            .map(|bit| {
                if bit * 2 < input.len() {
                    return 1;
                }

                0
            })
            .collect(),
    }
}

fn part_two(input: &[Binary]) -> usize {
    let oxigen: usize = calc_oxigen(input, None);
    let co2: usize = calc_co2(input, None);

    co2 * oxigen
}

fn calc_oxigen(list_to_check: &[Binary], current_index: Option<usize>) -> usize {
    if list_to_check.len() == 1 {
        return list_to_check.get(0).unwrap().clone().into();
    }

    let most_common = get_most_common(list_to_check);
    let index = current_index.unwrap_or(0);
    let bit = most_common.split.get(index).unwrap();
    let remaining_bits = list_to_check
        .iter()
        .cloned()
        .filter(|binary| binary.split[index] == *bit)
        .collect::<Vec<Binary>>();

    calc_oxigen(&remaining_bits, Some(index + 1))
}

fn calc_co2(list_to_check: &[Binary], current_index: Option<usize>) -> usize {
    if list_to_check.len() == 1 {
        return list_to_check.get(0).unwrap().clone().into();
    }

    let least_common = get_least_common(list_to_check);
    let index = current_index.unwrap_or(0);
    let bit = least_common.split.get(index).unwrap();
    let remaining_bits = list_to_check
        .iter()
        .cloned()
        .filter(|binary| binary.split[index] == *bit)
        .collect::<Vec<Binary>>();

    calc_co2(&remaining_bits, Some(index + 1))
}

#[cfg(test)]
mod tests {
    use crate::day_three::get_most_common;
    use crate::day_three::Binary;

    #[test]
    fn it_should_count_most_common() {
        let result = get_most_common(&[
            Binary {
                split: vec![1, 1, 1, 1, 0],
            },
            Binary {
                split: vec![1, 0, 1, 1, 0],
            },
            Binary {
                split: vec![1, 0, 1, 1, 1],
            },
            Binary {
                split: vec![1, 0, 1, 0, 1],
            },
            Binary {
                split: vec![1, 1, 1, 0, 0],
            },
            Binary {
                split: vec![1, 0, 0, 0, 0],
            },
            Binary {
                split: vec![1, 1, 0, 0, 1],
            },
        ]);

        assert_eq!("10100", result.to_string())
    }
}
