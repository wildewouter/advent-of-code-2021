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
    println!("Part two: {}", part_two(&grid));
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

fn part_two(grid: &HashMap<(i32, i32), i32>) -> usize {
    let mut checked: Vec<(i32, i32)> = Vec::new();
    let mut basins: Vec<usize> = Vec::new();

    for (position, value) in grid.iter() {
        if checked.contains(position) {
            continue;
        }

        if value == &9 {
            checked.push(*position);
            continue;
        }

        let basin = check_grid(position, &mut Vec::new(), grid);
        basins.push(basin.len());
        checked.extend(&basin);
    }

    basins.sort();
    basins.iter().rev().take(3).product::<usize>()
}

fn check_grid(
    position: &(i32, i32),
    basin: &mut Vec<(i32, i32)>,
    grid: &HashMap<(i32, i32), i32>,
) -> Vec<(i32, i32)> {
    if basin.contains(position) {
        return basin.clone();
    }

    let value = grid.get(position).unwrap();

    if value == &9 {
        return basin.clone();
    }

    basin.push(*position);

    let mut neighbours_in_basin: Vec<(i32, i32)> = Vec::new();

    let (x, y) = *position;
    for coordinate in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if let Some(neighbour) = grid.get(&coordinate) {
            if neighbour == &9 {
                continue;
            }

            neighbours_in_basin.push(coordinate);
        }
    }

    for pos in neighbours_in_basin {
        check_grid(&pos, basin, grid);
    }

    basin.clone()
}
