use std::collections::HashSet;

fn main() {
    let nq = scan_line();
    let n: u16 = nq[0];
    let q: u16 = nq[1];

    let mut followers: Vec<HashSet<u16>> = vec![HashSet::new(); n as usize];

    for _ in 0..q {
        let query = scan_line();
        match query[..] {
            [0, x, y] => {
                followers[y as usize].insert(x);
            }
            [1, z] => {
                let mut follower: Vec<_> = followers[z as usize].iter().collect();
                if follower.is_empty() {
                    println!("No");
                } else {
                    follower.sort();
                    let s = follower
                        .iter()
                        .map(|&x| u16::to_string(x))
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{}", s);
                }
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
