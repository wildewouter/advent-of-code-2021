mod read;

fn main() {
    let content = read::file("day-1/input");

    let lines: Vec<usize> = content
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    println!("Part one: {}", part_one(&lines));
}

fn part_one(lines: &Vec<usize>) -> usize {
    lines
        .iter()
        .zip(lines.iter().skip(1))
        .fold(0, |total, (previous, next)| {
            if next > previous {
                return total + 1;
            }

            return total;
        })
}
