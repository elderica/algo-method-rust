use std::collections::HashMap;

fn main() {
    let n = scan_line();
    let n: usize = n[0];
    let s: Vec<String> = read_words();

    let mut counter: HashMap<String, usize> = HashMap::new();

    for w in s {
        let mut cs: Vec<char> = w.chars().collect();
        cs.sort_unstable();
        let e = counter.entry(cs.iter().collect()).or_default();
        *e += 1;
    }

    let all_pattern = n * (n - 1) / 2;
    let same_pattern: usize = counter.iter().map(|(_, &v)| v * (v - 1) / 2).sum();

    let answer: f64 = same_pattern as f64 / all_pattern as f64;
    println!("{}", answer);
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
