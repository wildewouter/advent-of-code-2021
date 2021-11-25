mod read;

fn main() {
    let content = read::file("input");

    let lines: Vec<String> = content.lines().map(|x| String::from(x)).collect();

    let day_one = day_one(&lines);
    let day_two = day_two(&lines);

    println!("Day one: {}", day_one);
    println!("Day two: {}", day_two);
}

fn day_one(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .filter_map(|line| compare(line, &lines))
        .fold(1, |result, x| result * x)
}

fn day_two(lines: &Vec<String>) -> usize {
    let number_lines: Vec<usize> = lines
        .iter()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    number_lines
        .iter()
        .filter_map(|line| compare_day_two(line, &number_lines))
        .fold(1, |result, x| result * x)
}

fn compare(line: &str, other_lines: &[String]) -> Option<usize> {
    let number = line.parse::<usize>().ok()?;

    for x in other_lines {
        let i = x.parse::<usize>().ok()?;
        if i + number == 2020 {
            return Some(i);
        }
    }

    None
}

fn compare_day_two(line: &usize, other_lines: &[usize]) -> Option<usize> {
    for line_2 in other_lines {
        for line_3 in other_lines {
            if (line + line_2 + line_3) == 2020 {
                return Some(line.clone());
            }
        }
    }

    None
}

