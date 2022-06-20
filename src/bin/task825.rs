fn main() {
    let mut s = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let line1: Vec<u32> = scan_line();
    let q = line1[0];
    for _ in 0..q {
        let qy: Vec<u32> = scan_line();
        match qy[0] {
            0 => {
                let k = qy[1] as usize;
                println!("{}", s[k]);
            }
            1 => {
                let k = qy[1] as usize;
                let v = qy[2];
                s[k] = v;
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
