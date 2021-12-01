use crate::read;

pub fn run() {
    let content = read::file("day-1/input");

    let lines: Vec<usize> = content
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    println!("Day one:");
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

fn part_one(lines: &Vec<usize>) -> usize {
    lines
        .iter()
        .zip(lines.iter().skip(1))
        .fold(0, |total, (previous, next)| {
            if next > previous {
                return total + 1;
            }

            total
        })
}

fn part_two(lines: &Vec<usize>) -> usize {
    let computed_lines = lines
        .iter()
        .zip(lines.iter().skip(1).zip(lines.iter().skip(2)))
        .map(|(item, (item_1, item_2))| item + item_1 + item_2)
        .collect();

    part_one(&computed_lines)
}
