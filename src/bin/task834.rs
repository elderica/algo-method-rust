use std::collections::VecDeque;

fn main() {
    read_line();
    let a: Vec<usize> = scan_line();
    let mut a = VecDeque::from(a);
    let q: usize = scan_line()[0];

    for _ in 0..q {
        let qy = scan_line();
        match qy[..] {
            [0, v] => a.push_front(v),
            [1, v] => a.push_back(v),
            [2, k] => match a.get(k) {
                Some(v) => println!("{}", v),
                None => println!("Error"),
            },
            _ => {}
        };
    }
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
