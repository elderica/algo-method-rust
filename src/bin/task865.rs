fn main() {
    let n = scan_line();
    let _n: usize = n[0];
    let a_n: Vec<usize> = scan_line();

    let m = 5 * 100000;
    let mut counter = vec![0usize; m + 1];
    a_n.iter().for_each(|a_i| counter[*a_i] += 1);

    let amax = 5 * 100000;
    let mut memo = vec![-1; amax + 1];
    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let query: Vec<usize> = scan_line();
        if let [x] = query[..] {
            if memo[x] != -1 {
                println!("{}", memo[x]);
            } else {
                let sum: usize = (0..=amax).step_by(x).map(|v| counter[v]).sum();
                memo[x] = sum as i32;
                println!("{}", sum);
            }
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
