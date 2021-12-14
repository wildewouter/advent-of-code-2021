use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let fish: Vec<usize> = read::file(&path)
        .split(",")
        .filter_map(|age_str| age_str.parse::<usize>().ok())
        .collect();

    println!("Day six");
    println!("Part one: {}", part_one(&fish, &80));
    println!("Part two: {}", part_one(&fish, &256));
}

fn part_one(lantern_fish: &[usize], days: &usize) -> usize {
    let mut ages: HashMap<usize, usize> = HashMap::new();

    for fish in lantern_fish {
        ages.insert(*fish, *ages.get(fish).unwrap_or(&0) + 1);
    }

    for _ in 0..*days {
        let day8 = *ages.get(&8).unwrap_or(&0);
        let day7 = *ages.get(&7).unwrap_or(&0);
        let day6 = *ages.get(&6).unwrap_or(&0);
        let day5 = *ages.get(&5).unwrap_or(&0);
        let day4 = *ages.get(&4).unwrap_or(&0);
        let day3 = *ages.get(&3).unwrap_or(&0);
        let day2 = *ages.get(&2).unwrap_or(&0);
        let day1 = *ages.get(&1).unwrap_or(&0);
        let day0 = *ages.get(&0).unwrap_or(&0);

        ages.insert(8, day0);
        ages.insert(7, day8);
        ages.insert(6, day7 + day0);
        ages.insert(5, day6);
        ages.insert(4, day5);
        ages.insert(3, day4);
        ages.insert(2, day3);
        ages.insert(1, day2);
        ages.insert(0, day1);
    }

    ages.values().fold(0, |x, y| x + y)
}
