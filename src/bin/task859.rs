use std::vec;

fn main() {
    let mut counter = vec![0usize; 500001];
    for x in -100..=100 {
        for y in -100..=100 {
            let v: i32 = x * x + y * y;
            counter[v as usize] += 1;
        }
    }

    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let query: Vec<usize> = scan_line();
        if let [p] = query[..] {
            println!("{}", counter[p]);
        }
    }
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
