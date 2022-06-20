fn main() {
    let n = scan_line();
    let _n: usize = n[0];

    let numbers: Vec<usize> = scan_line();
    let nmax = 5 * 10usize.pow(5) + 1;
    let mut counts = vec![0; nmax];
    for x in numbers {
        counts[x] += 1;
    }

    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let query: Vec<usize> = scan_line();
        match *query {
            [0, v] => counts[v] += 1,
            [1, v] => counts[v] = 0,
            [2, v] => {
                println!("{}", counts[v])
            }
            _ => {}
        }
    }
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
