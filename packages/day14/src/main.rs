use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content = read::file(&path);
    let (input, rules) = content.split_once("\n\n").unwrap();

    let mut rule_set: HashMap<String, String> = HashMap::new();

    rules.lines().for_each(|line| {
        let line: Vec<&str> = line.split(" -> ").collect();
        let pair = line[0];
        let result = line[1];

        rule_set.insert(pair.to_string(), result.to_string());
    });

    let start_set = input.split("").filter(|char| char != &"").collect();

    println!("Day fourteen");
    println!("Part one: {}", do_it(&start_set, &rule_set, 10));
    println!("Part two: {}", do_it(&start_set, &rule_set, 40));
}

fn do_it(start_set: &Vec<&str>, rule_set: &HashMap<String, String>, times: usize) -> usize {
    let mut pairs_in_line: HashMap<String, usize> = HashMap::new();

    for pair in start_set.windows(2) {
        let index = format!("{}{}", pair[0], pair[1]);
        let in_set = pairs_in_line.get(&index).unwrap_or(&0).clone();
        pairs_in_line.insert(index.clone(), in_set + &1);
    }

    for _ in 0..times {
        let mut pairs_in_next_line: HashMap<String, usize> = HashMap::new();

        for (pair, amount) in pairs_in_line.clone() {
            if let Some(letter) = rule_set.get(&pair.to_string()) {
                let existing: Vec<char> = pair.chars().collect();
                for combo in vec![
                    &format!("{}{}", existing[0], letter),
                    &format!("{}{}", letter, existing[1]),
                ] {
                    pairs_in_next_line.insert(
                        combo.to_string(),
                        pairs_in_next_line.get(combo).unwrap_or(&0) + amount,
                    );
                }
            }
        }

        pairs_in_line = pairs_in_next_line;
    }

    let mut result_set: HashMap<String, usize> = HashMap::new();
    result_set.insert(start_set[0].to_string(), 1);

    for (pair, value) in pairs_in_line {
        let letters: Vec<char> = pair.chars().collect();

        let string = letters[1].to_string();
        result_set.insert(
            string.clone(),
            result_set.get(&string).unwrap_or(&0) + value,
        );
    }

    result_set.values().max().unwrap() - result_set.values().min().unwrap()
}
