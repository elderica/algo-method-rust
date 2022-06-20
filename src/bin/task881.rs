use std::collections::HashSet;

fn main() {
    let _n = scan_line();
    let _n: usize = _n[0];
    let _a: Vec<usize> = scan_line();

    let set: HashSet<usize> = _a.into_iter().collect();

    println!("{}", _n - set.len());
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

#[allow(dead_code)]
fn read_words() -> Vec<String> {
    read_line().split_whitespace().map(String::from).collect()
}

#[allow(dead_code)]
fn scan_line<F>() -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line()
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}
