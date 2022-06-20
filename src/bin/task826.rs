fn main() {
    let line1: Vec<usize> = scan_line();
    // let n = line1[0];
    let k = line1[1];
    let a: Vec<usize> = scan_line();

    println!("{}", a[k]);
    println!("{}", a[a.len() - k - 1]);
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
