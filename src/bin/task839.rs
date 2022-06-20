fn main() {
    let nq = scan_line();
    let _n: usize = nq[0];
    let q: usize = nq[1];

    let mut followers = vec![vec![false; 5001]];
    let mut followercounts = vec![0usize, 5001];

    for _ in 0..q {
        let query: Vec<usize> = scan_line();
        match query[..] {
            [0, x, y] => {
                if !followers[y][x] {
                    followers[y][x] = true;
                    followercounts[y] += 1;
                }
            }
            [1, x, y] => {
                if followers[y][x] {
                    followers[y][x] = false;
                    followercounts[y] -= 1;
                }
            }
            [2, z] => {
                println!("{}", followercounts[z]);
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
