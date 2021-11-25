mod read;

fn main() {
    let content = read::file("input");

    let lines: Vec<String> = content.lines().map(|x| String::from(x)).collect();

    let result = lines
        .iter()
        .filter_map(|line| compare(line, &lines))
        .fold(1, |result, x| result * x);

    println!("{:?}", result);
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
