use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content: Vec<Vent> = read::file(&path)
        .lines()
        .map(|line| line.parse::<Vent>().unwrap())
        .collect();
    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

fn part_one(vents: &[Vent]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    let straight_vents: Vec<&Vent> = vents
        .iter()
        .filter(|vent| vent.a.0 == vent.b.0 || vent.a.1 == vent.b.1)
        .collect();

    count_straight_vents(&mut map, &straight_vents);

    map.values().fold(0, |total, lines_at_position| {
        if lines_at_position < &2 {
            return total;
        }

        total + 1
    })
}

fn part_two(vents: &[Vent]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    let straight_vents: Vec<&Vent> = vents
        .iter()
        .filter(|vent| vent.a.0 == vent.b.0 || vent.a.1 == vent.b.1)
        .collect();

    count_straight_vents(&mut map, &straight_vents);

    let diagonal_vents: Vec<&Vent> = vents
        .iter()
        .filter(|vent| vent.a.0 != vent.b.0 && vent.a.1 != vent.b.1)
        .collect();

    count_diagonal_vents(&mut map, &diagonal_vents);

    map.values().fold(0, |total, lines_at_position| {
        if lines_at_position < &2 {
            return total;
        }

        total + 1
    })
}

fn count_straight_vents(map: &mut HashMap<(usize, usize), usize>, straight_vents: &[&Vent]) {
    for vent in straight_vents {
        let x_smallest_value = ordered_value(vent.a.0, vent.b.0).0;
        let x_diff = abs_difference(vent.a.0, vent.b.0);

        for x in x_smallest_value..=(x_smallest_value + x_diff) {
            let y_smallest_value = ordered_value(vent.a.1, vent.b.1).0;
            let y_diff = abs_difference(vent.a.1, vent.b.1);

            for y in y_smallest_value..=(y_smallest_value + y_diff) {
                map.insert((x, y), map.get(&(x, y)).unwrap_or(&0) + 1);
            }
        }
    }
}

fn count_diagonal_vents(map: &mut HashMap<(usize, usize), usize>, diagonal_vents: &[&Vent]) {
    for vent in diagonal_vents {
        let mut x_direction = 1;

        if vent.a.0 > vent.b.0 {
            x_direction = -1;
        }

        let mut y_direction = 1;

        if vent.a.1 > vent.b.1 {
            y_direction = -1;
        }

        change_position(vent.a, vent.b, x_direction, y_direction, map);
    }
}

fn change_position(
    position: (usize, usize),
    end_position: (usize, usize),
    x_direction: i32,
    y_direction: i32,
    map: &mut HashMap<(usize, usize), usize>,
) {
    map.insert(position, map.get(&position).unwrap_or(&0) + 1);

    if position == end_position {
        return;
    }

    let next_position = (
        (position.0 as i32 + x_direction) as usize,
        (position.1 as i32 + y_direction) as usize,
    );

    change_position(next_position, end_position, x_direction, y_direction, map)
}

fn ordered_value(x: usize, y: usize) -> (usize, usize) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

fn abs_difference(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[derive(Debug)]
struct Vent {
    a: (usize, usize),
    b: (usize, usize),
}

impl Vent {
    fn new(a: (usize, usize), b: (usize, usize)) -> Self {
        Vent { a, b }
    }
}

impl FromStr for Vent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();

        match rx.captures(s) {
            Some(captures) => Ok(Vent::new(
                (
                    captures["x1"].parse::<usize>().unwrap(),
                    captures["y1"].parse::<usize>().unwrap(),
                ),
                (
                    captures["x2"].parse::<usize>().unwrap(),
                    captures["y2"].parse::<usize>().unwrap(),
                ),
            )),
            None => Err(()),
        }
    }
}
