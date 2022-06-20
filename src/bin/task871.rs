use std::collections::HashSet;

fn main() {
    let nq = scan_line();
    let n: usize = nq[0];
    let q: usize = nq[1];

    let mut followers_of: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    for _ in 0..q {
        let query = scan_line();
        match query[..] {
            [0, x, y] => {
                followers_of[y].insert(x);
            }
            [1, x, y] => {
                followers_of[y].remove(&x);
            }
            [2, z] => {
                let except_z = (0..z).chain((z + 1)..followers_of.len());
                let n_same = except_z
                    .filter(|i| followers_of[z] == followers_of[*i])
                    .count();
                println!("{}", n_same);
            }
            _ => {}
        };
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

#[allow(dead_code)]
fn read_words() -> Vec<String> {
    read_line().split_whitespace().map(String::from).collect()
}

#[allow(dead_code)]
fn scan_line<F>() -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line()
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}
