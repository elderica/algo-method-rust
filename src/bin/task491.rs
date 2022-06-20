fn main() {
    let nk = scan_line();
    let n: usize = nk[0];
    let k: usize = nk[1];

    let mut foods: Vec<(usize, usize)> = Vec::new();
    foods.reserve(n);
    (0..n).for_each(|_| {
        if let [a, b] = scan_line()[..] {
            foods.push((a, b));
        }
    });
    foods.sort_unstable_by(|p, q| p.0.cmp(&q.0));

    let mut sum = 0;
    let mut left = k;
    for food in foods {
        if left == 0 {
            break;
        }
        let c = food.1.min(left);
        sum += food.0 * c;
        left -= c;
    }
    println!("{}", sum);
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
