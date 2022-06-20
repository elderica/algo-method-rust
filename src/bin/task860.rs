fn main() {
    let n = scan_line();
    let _n: usize = n[0];

    let numbers: Vec<usize> = scan_line();
    let nmax = 5 * 10usize.pow(5) + 1;
    let mut counts = vec![0; nmax];
    numbers.iter().for_each(|s| counts[*s] += 1);
    let &max = counts.iter().max().unwrap();

    println!("{}", (0usize..nmax).find(|i| counts[*i] == max).unwrap());
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
