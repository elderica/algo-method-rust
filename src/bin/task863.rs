fn main() {
    let n = scan_line();
    let _n: usize = n[0];
    let an: Vec<usize> = scan_line();

    let mut sum: usize = an.iter().sum();
    let mut counter = vec![0usize; 200001];
    for x in an {
        counter[x] += 1;
    }

    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let query: Vec<usize> = scan_line();
        match query[..] {
            [0, v] => {
                sum += v;
                counter[v] += 1;
            }
            [1, x, y] => {
                let cx = counter[x];
                counter[x] = 0;
                sum -= x * cx;
                counter[y] += cx;
                sum += y * cx;
            }
            [2] => {
                println!("{}", sum);
            }
            _ => {}
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
