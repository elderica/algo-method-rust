use std::collections::VecDeque;

fn main() {
    read_line();
    let a: Vec<usize> = scan_line();
    let mut deq = VecDeque::from(a);
    let q: usize = scan_line()[0];

    for _ in 0..q {
        let qy = scan_line();
        match qy[0] {
            0usize => {
                let v = qy[1];
                deq.push_back(v);
            }
            1usize => {
                let v = deq.pop_back();
                match v {
                    Some(v) => println!("{}", v),
                    None => println!("Error"),
                }
            }
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
