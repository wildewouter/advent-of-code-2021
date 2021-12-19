use std::alloc::System;
use std::collections::HashMap;
use std::process::exit;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content = read::file(&path);
    let (input, rules) = content.split_once("\n\n").unwrap();

    let mut rule_set: HashMap<(String), String> = HashMap::new();

    rules.lines().for_each(|line| {
        let line: Vec<&str> = line.split(" -> ").collect();
        let pair = line[0];
        let result = line[1];

        rule_set.insert(pair.to_string(), result.to_string());
    });

    println!("Day fourteen");
    let start_set = input.split("").filter(|char| char != &"").collect();
    println!("Part one: {}", do_it(&start_set, &rule_set, 10));
    println!("Part two: {}", do_it(&start_set, &rule_set, 40));
}

fn do_it(start_set: &Vec<&str>, rule_set: &HashMap<(String), String>, times: usize) -> usize {
    let mut pair_set: HashMap<String, usize> = HashMap::new();

    for pair in start_set.windows(2) {
        let index = format!("{}{}", pair[0], pair[1]);
        let in_set = pair_set.get(&index).unwrap_or(&0);
        pair_set.insert(index.clone(), in_set + &1);
    }

    for _ in 0..times {
        let mut temp_set: HashMap<String, usize> = HashMap::new();

        for (pair, amount) in pair_set.clone() {
            if let Some(letter) = rule_set.get(&pair.to_string()) {
                let existing: Vec<char> = pair.chars().take(2).collect();
                for combo in vec![
                    &format!("{}{}", existing[0], letter),
                    &format!("{}{}", letter, existing[1]),
                ] {
                    temp_set.insert(
                        combo.to_string(),
                        temp_set.get(combo).unwrap_or(&0) + amount,
                    );
                }
            }
        }

        pair_set = temp_set;
    }

    let mut result_set: HashMap<String, usize> = HashMap::new();
    result_set.insert("N".to_string(), 1);

    for (pair, value) in pair_set {
        let letters: Vec<char> = pair.chars().take(2).collect();

        let string = letters[1].to_string();
        result_set.insert(
            string.clone(),
            result_set.get(&string).unwrap_or(&0) + value,
        );
    }

    result_set.values().max().unwrap() - result_set.values().min().unwrap()
}
