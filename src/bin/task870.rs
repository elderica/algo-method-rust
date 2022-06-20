use std::collections::HashMap;

fn main() {
    read_line();
    let words = read_words();
    let mut counter: HashMap<usize, usize> = HashMap::new();
    for w in words {
        let h = hash(&w);
        let c = counter.entry(h).or_default();
        *c += 1;
    }
    let m = counter.values().max().unwrap_or(&0);
    println!("{}", m);
}

const B: usize = 30;
const M: usize = 1000003;

fn hash(s: &str) -> usize {
    let a2i: HashMap<char, usize> = ('a'..='z').zip(1..).collect();
    let rcs = s.chars().rev().zip(0u32..);
    let s: usize = rcs.map(|(c, i)| a2i[&c] * B.pow(i)).sum();
    s % M
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
