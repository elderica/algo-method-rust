fn main() {
    read_line();
    let mut a: Vec<usize> = scan_line();
    let q: usize = scan_line()[0];

    for _ in 0..q {
        let qy = scan_line();
        match qy[0] {
            0usize => {
                let k = qy[1];
                match a.get(k) {
                    Some(&v) => println!("{}", v),
                    None => println!("Error"),
                }
            }
            1usize => {
                let v = qy[1];
                a.push(v);
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
