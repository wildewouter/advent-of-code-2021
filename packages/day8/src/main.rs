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
