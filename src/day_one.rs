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

fn part_one(lines: &[usize]) -> usize {
    lines.windows(2).fold(0, |total, pair| match pair {
        [previous, next] => {
            if next > previous {
                return total + 1;
            }

            total
        }
        _ => total,
    })
}

fn part_two(lines: &[usize]) -> usize {
    let computed_lines: Vec<usize> = lines.windows(3).map(|items| items.iter().sum()).collect();

    part_one(&computed_lines)
}
