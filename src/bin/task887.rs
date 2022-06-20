fn main() {
    let _n = scan_line();
    let _q = read_line();
    let mut open_brackets = Vec::with_capacity(_n[0]);

    for (i, c) in _q.char_indices() {
        match c {
            '(' => open_brackets.push(i),
            ')' => {
                if let Some(0) = open_brackets.pop() {
                    println!("{}", i);
                }
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
