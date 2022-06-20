use std::collections::HashMap;

fn main() {
    let n = scan_line();
    let _n: usize = n[0];
    let s_n: Vec<String> = read_words();

    let mut counter: HashMap<String, usize> = HashMap::new();
    s_n.into_iter().for_each(|w| {
        let c = counter.entry(w).or_default();
        *c += 1;
    });

    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let query = read_line();
        let c = counter.entry(query).or_default();
        println!("{:?}", c);
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
