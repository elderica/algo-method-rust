use std::collections::HashSet;

fn main() {
    let nm = scan_line();
    let _n: usize = nm[0];
    let _m: usize = nm[1];
    let an: Vec<usize> = scan_line();

    let mut sum_square: HashSet<usize> = HashSet::new();
    for a in an.iter() {
        for b in an.iter() {
            let v = *a * *a + *b * *b;
            sum_square.insert(v);
        }
    }
    let mut probable = "No";
    for e in &sum_square {
        if sum_square.contains(&(_m - *e)) {
            probable = "Yes";
        }
    }
    println!("{}", probable);
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
