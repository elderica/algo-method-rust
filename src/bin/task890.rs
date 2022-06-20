fn main() {
    #[allow(unused)]
    let n = scan_line()[0];
    let s = read_line();
    let mut open_brackets = Vec::with_capacity(n);
    let mut corresponding_brackets = vec![0_usize; n];

    for (right, c) in s.char_indices() {
        match c {
            '(' => open_brackets.push(right),
            ')' => {
                if let Some(left) = open_brackets.pop() {
                    corresponding_brackets[left] = right;
                    corresponding_brackets[right] = left;
                }
            }
            _ => {}
        }
    }

    let _q = scan_line()[0];
    for _ in 0.._q {
        let k: usize = scan_line()[0];
        println!("{}", corresponding_brackets[k]);
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
