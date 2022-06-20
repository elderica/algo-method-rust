use std::collections::LinkedList;

fn main() {
    let q = scan_line();
    let q: usize = q[0];

    let mut list = LinkedList::new();

    for _ in 0..q {
        let query = read_words();
        let op = query[0].to_string();
        let word = query[1].to_string();
        match &*op {
            "0" => {
                list.push_front(word.to_string());
            }
            "1" => {
                let n = word.as_str().parse().unwrap();
                list.iter().take(n).for_each(|w| print!("{} ", *w));
                println!();
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
