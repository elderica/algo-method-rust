use std::collections::HashMap;

fn main() {
    let n = scan_line();
    let _n: usize = n[0];

    let alphabet: HashMap<char, usize> = ('a'..='z').zip(0..).collect();

    let words: Vec<String> = read_words();
    let mut counts = vec![false; 26];
    for w in words {
        for b in w.chars() {
            let n = alphabet[&b];
            counts[n] = true;
        }
    }

    println!(
        "{}",
        if counts.iter().all(|b| *b) {
            "Yes"
        } else {
            "No"
        }
    );
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

#[allow(dead_code)]
fn read_words() -> Vec<String> {
    read_line()
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect()
}

#[allow(dead_code)]
fn scan_line<F>() -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line()
        .trim()
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}
