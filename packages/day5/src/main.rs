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
}

fn part_one(vents: &[Vent]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    for vent in vents {
        if vent.a.0 != vent.b.0 && vent.a.1 != vent.b.1 {
            continue;
        }

        for x in vent.a.0..=vent.b.0 {
            for y in vent.a.1..=vent.b.1 {
                map.insert((x, y), map.get(&(x, y)).unwrap_or(&0) + 1);
            }
        }
    }

    map.values().fold(0, |total, lines_at_position| {
        if lines_at_position < &2 {
            return total;
        }

        total + 1
    })
}

#[derive(Debug)]
struct Vent {
    a: (usize, usize),
    b: (usize, usize),
}

impl FromStr for Vent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();

        match rx.captures(s) {
            Some(captures) => {
                let mut x = [
                    captures["x1"].parse::<usize>().unwrap(),
                    captures["x2"].parse::<usize>().unwrap(),
                ];
                x.sort_unstable();

                let mut y = [
                    captures["y1"].parse::<usize>().unwrap(),
                    captures["y2"].parse::<usize>().unwrap(),
                ];
                y.sort_unstable();

                Ok(Vent {
                    a: (x[0], y[0]),
                    b: (x[1], y[1]),
                })
            }
            None => Err(()),
        }
    }
}
