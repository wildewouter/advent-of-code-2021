fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let content = read::file(&path)
        .lines()
        .map(|line| line.split(" -> "))
        .map(|set| set.map(|num| num.parse::<usize>()))
        .collect();
}

// .split(" -> ")
// .into_iter()
// .map(|set| {
// let set: Vec<usize> = String::from(set)
// .split(',')
// .iter()
// .map(|coordinate| coordinate.parse::<usize>())
// .collect();
// });
