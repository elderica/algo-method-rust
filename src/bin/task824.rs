fn main() {
    let line1: Vec<u32> = scan_line();
    solve(line1[0] as usize);
}

fn solve(k: usize) {
    let a = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("{}", a[k]);
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
