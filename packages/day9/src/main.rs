use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for (x, line) in read::file(&path).lines().enumerate() {
        for (y, value) in line.split("").filter(|char| !char.is_empty()).enumerate() {
            grid.insert((x as i32, y as i32), value.parse::<i32>().unwrap());
        }
    }

    println!("Day nine");
    println!("Part one: {}", part_one(&grid));
}

fn part_one(grid: &HashMap<(i32, i32), i32>) -> usize {
    let mut low_points: Vec<usize> = Vec::new();

    for ((x, y), value) in grid.iter() {
        if grid.get(&(x - 1, *y)).unwrap_or(&(value + 1)) > value
            && grid.get(&(x + 1, *y)).unwrap_or(&(value + 1)) > value
            && grid.get(&(*x, y - 1)).unwrap_or(&(value + 1)) > value
            && grid.get(&(*x, y + 1)).unwrap_or(&(value + 1)) > value
        {
            low_points.push(1 + *value as usize);
        }
    }

    low_points.iter().sum()
}
