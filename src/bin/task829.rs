fn main() {
    read_line();
    let mut a: Vec<usize> = scan_line();
    let q: usize = scan_line()[0];

    for _ in 0..q {
        let qy = scan_line();
        match qy[..] {
            [0, k, v] => a.insert(k, v),
            [1, k] => {
                a.remove(k);
            }
            [2, v] => {
                let c = a.iter().filter(|&&w| v == w).count();
                println!("{}", c);
            }
            _ => {}
        };
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

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
