fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let fish: Vec<usize> = read::file(&path)
        .split(",")
        .filter_map(|age_str| age_str.parse::<usize>().ok())
        .collect();

    println!("Part one: {}", part_one(&fish, &80));
}

fn part_one(lantern_fish: &[usize], days: &usize) -> usize {
    pass_day(lantern_fish, &0, days).len()
}

fn pass_day(lantern_fish: &[usize], current_day: &usize, end_day: &usize) -> Vec<usize> {
    if current_day == end_day {
        return Vec::from(lantern_fish);
    }

    let new_fish: Vec<usize> = lantern_fish
        .into_iter()
        .map(produce_fish)
        .flatten()
        .collect();

    pass_day(&new_fish, &(current_day + 1), end_day)
}

fn produce_fish(fish: &usize) -> Vec<usize> {
    if fish == &0 {
        return Vec::from([6, 8]);
    }

    Vec::from([fish - 1])
}
