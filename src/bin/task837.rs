use std::collections::HashMap;

fn main() {
    let line1 = read_words();
    let n: usize = line1[0].parse().unwrap();
    let colortocount = &line1[1];

    let mut studentcolor: HashMap<String, u32> = HashMap::new();
    for _ in 1..=n {
        let qy = read_words();
        for color in &qy[1..] {
            let c = studentcolor.entry(color.clone()).or_insert(0);
            *c += 1;
        }
    }
    let c = studentcolor.get(colortocount).unwrap_or(&0);
    println!("{}", *c);
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

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
