use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content: Vec<usize> = read::file(&path)
        .split(',')
        .into_iter()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();

    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

fn part_one(crabs: &[usize]) -> usize {
    let min = crabs.into_iter().min().unwrap();
    let max = crabs.into_iter().max().unwrap();

    let mut fuel_used: HashMap<usize, usize> = HashMap::new();
    for new_position in *min..*max {
        for crabby in crabs {
            fuel_used.insert(
                new_position,
                fuel_used.get(&new_position).unwrap_or(&0) + abs_difference(&new_position, crabby),
            );
        }
    }

    *fuel_used.values().min().unwrap()
}

fn part_two(crabs: &[usize]) -> usize {
    let min = crabs.into_iter().min().unwrap();
    let max = crabs.into_iter().max().unwrap();

    let mut fuel_used: HashMap<usize, usize> = HashMap::new();
    for new_position in *min..*max {
        for crabby in crabs {
            let range = 1..(abs_difference(&new_position, crabby) + 1);
            let fuel: usize = range.into_iter().sum();

            fuel_used.insert(
                new_position,
                fuel_used.get(&new_position).unwrap_or(&0) + fuel,
            );
        }
    }

    *fuel_used.values().min().unwrap()
}

fn abs_difference(x: &usize, y: &usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}
