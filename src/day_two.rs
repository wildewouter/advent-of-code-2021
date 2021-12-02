use crate::read;

pub fn run() {
    let content: Vec<(String, usize)> = read::file("day-2/input")
        .lines()
        .map(|line| {
            let mut split_line = line.split_whitespace();
            let direction: String = split_line.next().unwrap().to_string();
            let amount: usize = split_line.next().unwrap().parse::<usize>().unwrap();

            (direction, amount)
        })
        .collect();

    println!("Day two:");
    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

fn part_one(movement: &[(String, usize)]) -> usize {
    let new_coordinates = movement.iter().fold((0, 0), |coordinates, (direction, amount)| {
        let (x, y): (usize, usize) = match direction.as_ref() {
            "forward" => {
                (coordinates.0, coordinates.1 + amount)
            }
            "down" => {
                (coordinates.0 + amount, coordinates.1)
            }
            "up" => {
                (coordinates.0 - amount, coordinates.1)
            }
            _ => coordinates,
        };

        (x, y)
    });

    new_coordinates.0 * new_coordinates.1
}

fn part_two(movement: &[(String, usize)]) -> usize {
    let new_coordinates = movement.iter().fold((0, 0, 0), |coordinates, (direction, amount)| {
        let (x, y, z): (usize, usize, usize) = match direction.as_ref() {
            "forward" => {
                (coordinates.0 + (amount * coordinates.2), coordinates.1 + amount, coordinates.2)
            }
            "down" => {
                (coordinates.0, coordinates.1, coordinates.2 + amount)
            }
            "up" => {
                (coordinates.0, coordinates.1, coordinates.2 - amount)
            }
            _ => coordinates,
        };

        (x, y, z)
    });

    new_coordinates.0 * new_coordinates.1
}