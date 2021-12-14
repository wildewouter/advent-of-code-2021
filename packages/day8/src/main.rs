use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content: Vec<(String, String)> = read::file(&path)
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split('|').into_iter().collect();

            (split[0].to_string(), split[1].to_string())
        })
        .collect();

    println!("Day eight");
    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

fn part_one(input: &[(String, String)]) -> usize {
    input
        .iter()
        .map(|(_, str)| {
            str.split_whitespace()
                .into_iter()
                .filter(|sequence| matches!(sequence.chars().count(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn part_two(input: &[(String, String)]) -> usize {
    input
        .iter()
        .map(|(wiring, actual)| {
            let list: HashMap<usize, &str> = determine_unique_length_nums(wiring);

            let func = determine_number(
                list.get(&1).unwrap().to_string(),
                list.get(&4).unwrap().to_string(),
                list.get(&7).unwrap().to_string(),
            );

            actual
                .split_whitespace()
                .into_iter()
                .fold(String::new(), |result, sequence| {
                    if let Some(number) = func(sequence) {
                        return format!("{}{}", result, number);
                    }

                    result
                })
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn determine_unique_length_nums(wiring: &str) -> HashMap<usize, &str> {
    let mut list: HashMap<usize, &str> = HashMap::new();

    for sequence in wiring.split_whitespace() {
        match sequence.chars().count() {
            2 => list.insert(1, sequence.clone()),
            3 => list.insert(7, sequence.clone()),
            4 => list.insert(4, sequence.clone()),
            _ => None,
        };
    }

    list
}

fn determine_number(one: String, four: String, seven: String) -> impl Fn(&str) -> Option<usize> {
    return move |number: &str| -> Option<usize> {
        let one_vec = one
            .split("")
            .filter(|char| char.len() > 0)
            .collect::<Vec<&str>>();
        let four_vec = four
            .split("")
            .filter(|char| char.len() > 0)
            .collect::<Vec<&str>>();
        let seven_vec = seven
            .split("")
            .filter(|char| char.len() > 0)
            .collect::<Vec<&str>>();

        match number.chars().count() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            5 => {
                let is_3 = does_apply(&Vec::from([&one_vec, &seven_vec]), number);

                if is_3 {
                    return Some(3);
                }

                if number
                    .split("")
                    .filter(|char| char.len() > 0)
                    .filter(|&char| four_vec.contains(&char))
                    .count()
                    == 3
                {
                    return Some(5);
                }

                Some(2)
            }
            6 => {
                let is_9 = does_apply(&Vec::from([&one_vec, &four_vec, &seven_vec]), number);

                if is_9 {
                    return Some(9);
                }

                let is_0 = does_apply(&Vec::from([&one_vec, &seven_vec]), number);

                if is_0 {
                    return Some(0);
                }

                Some(6)
            }
            7 => Some(8),
            _ => None,
        }
    };
}

fn does_apply(vecs_to_check: &[&Vec<&str>], number: &str) -> bool {
    vecs_to_check.into_iter().fold(true, |result, next_vec| {
        let remaining: Vec<&str> = number
            .split("")
            .filter(|char| char.len() > 0)
            .filter(|char| next_vec.contains(&char))
            .collect();

        result && remaining.len() == next_vec.len()
    })
}

#[cfg(test)]
mod tests {
    use crate::{determine_number, determine_unique_length_nums};
    use std::collections::HashMap;

    #[test]
    fn it_should_work() {
        let wiring = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let input0 = "cdfeb";
        let input1 = "fcadb";
        let input2 = "cdbaf";
        let input3 = "cdfgeb";

        let list: HashMap<usize, &str> = determine_unique_length_nums(wiring);

        let func = determine_number(
            list.get(&1).unwrap().to_string(),
            list.get(&4).unwrap().to_string(),
            list.get(&7).unwrap().to_string(),
        );

        assert_eq!(5, func(input0).unwrap());
        assert_eq!(3, func(input1).unwrap());
        assert_eq!(3, func(input2).unwrap());
        assert_eq!(6, func(input3).unwrap());
    }
}
