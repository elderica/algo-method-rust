fn main() {
    let n = scan_line();
    let n: usize = n[0];
    let an: Vec<usize> = scan_line();

    let mut counter = vec![0usize; 200001];
    an.into_iter().for_each(|a| counter[a] += 1);

    let all_pattern = n * (n - 1) / 2;
    let mut same_pattern = 0usize;
    (0..200000).for_each(|i| {
        let c = counter[i];
        same_pattern += c * (c - 1) / 2;
    });

    println!("{}", same_pattern as f64 / all_pattern as f64);
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
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
