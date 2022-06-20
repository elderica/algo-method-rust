fn main() {
    let q: Vec<usize> = scan_line();
    let n = q[0];
    let q = q[1];

    let nil = usize::MAX;
    let mut next: Vec<usize> = vec![nil; n];
    let mut prev: Vec<usize> = vec![nil; n];

    for _ in 0..q {
        let query: Vec<usize> = scan_line();

        match *query {
            [0, p, q] => {
                next[p] = q;
                prev[q] = p;
            }
            [1, r] => {
                if next[r] != nil {
                    prev[next[r]] = prev[r];
                }
                if prev[r] != nil {
                    next[prev[r]] = next[r];
                }
                next[r] = nil;
                prev[r] = nil;
            }
            _ => {}
        }
    }

    let mut count = -1;
    let mut current = 0usize;
    while current != nil {
        count += 1;
        current = next[current];
    }
    current = 0usize;
    while current != nil {
        count += 1;
        current = prev[current];
    }
    println!("{}", count);
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
