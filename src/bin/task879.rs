use std::collections::HashSet;

fn main() {
    read_line();
    println!("{}", read_words().into_iter().collect::<HashSet<_>>().len());
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
