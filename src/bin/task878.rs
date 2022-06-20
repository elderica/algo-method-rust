use std::collections::HashSet;

fn main() {
    let mut words: HashSet<String> = HashSet::new();
    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let mut query = read_words();
        let t = query.pop().unwrap();
        let op = query.pop().unwrap();
        match op.as_str() {
            "0" => {
                words.insert(t);
            }
            "1" => {
                words.remove(&t);
            }
            "2" => {
                println!("{}", if words.contains(&t) { "Yes" } else { "No" })
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
