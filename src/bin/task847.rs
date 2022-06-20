use std::collections::VecDeque;

fn main() {
    let q = scan_line();
    let q: usize = q[0];

    let mut deque = VecDeque::new();

    for _ in 0..q {
        let query = read_words();
        let op = query[0].to_string();

        match &*op {
            "0" => {
                let word = query[1].clone();
                deque.push_front(word);
            }
            "1" => {
                let word = query[1].clone();
                deque.push_back(word);
            }
            "2" => {
                let word = Option::unwrap_or(deque.pop_front(), String::from("Error"));
                println!("{}", word);
            }
            "3" => {
                let word = Option::unwrap_or(deque.pop_back(), String::from("Error"));
                println!("{}", word);
            }
            _ => {}
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
