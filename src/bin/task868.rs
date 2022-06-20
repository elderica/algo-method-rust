use std::collections::HashMap;

fn main() {
    read_line();
    let words = read_words();
    let mut wordcounts: HashMap<String, usize> = HashMap::new();
    words.into_iter().for_each(|w| {
        let c = wordcounts.entry(w).or_default();
        *c += 1;
    });
    // println!("{:?}", wordcounts);

    let q = scan_line();
    let q: usize = q[0];
    for _ in 0..q {
        let mut query = read_words();
        let t = query.pop().unwrap();
        let op = query.pop().unwrap();
        // println!("{:?}", wordcounts);
        let c = wordcounts.entry(t).or_default();
        match op.as_str() {
            "0" => {
                *c += 1;
            }
            "1" => {
                *c = 0;
            }
            "2" => {
                println!("{}", *c);
            }
            _ => {}
        }
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
