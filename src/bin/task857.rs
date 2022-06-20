fn main() {
    #[allow(unused)]
    let n: usize = scan_line()[0];

    let scores: Vec<usize> = scan_line();
    let mut counts = vec![0; 100001];
    scores.iter().for_each(|s| counts[*s] += 1);

    let q = scan_line();
    let q: usize = q[0];

    (0..q).for_each(|_| {
        let query: Vec<usize> = scan_line();
        if let [s] = *query {
            println!("{}", counts[s]);
        }
    });
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
