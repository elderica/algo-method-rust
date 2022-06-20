use std::collections::VecDeque;

fn main() {
    let q = scan_line();
    let q: usize = q[0];

    let mut vec = VecDeque::new();

    for _ in 0..q {
        let query = read_words();
        let op = query[0].to_string();

        if op == "0" {
            let word = query[1].to_string();
            vec.push_back(word);
        } else if op == "1" {
            let word = Option::unwrap_or(vec.pop_front(), String::from("Error"));
            println!("{}", word);
        }
    }
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
