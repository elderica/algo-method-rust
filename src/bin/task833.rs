fn main() {
    let n = scan_line();
    let n: usize = n[0];

    let mut a: Vec<usize> = (1..=n).collect();

    while a.len() > 1 {
        a.remove(0);
        a.push(a[0]);
        a.remove(0);
    }

    println!("{}", a[0]);
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
