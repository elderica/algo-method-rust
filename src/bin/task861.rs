fn main() {
    let nm = scan_line();
    let _n: usize = nm[0];
    let m: usize = nm[1];
    let an: Vec<usize> = scan_line();

    let mut is_sum_square = vec![false; 2000001];
    for a in an.iter() {
        for b in an.iter() {
            let v = *a * *a + *b * *b;
            is_sum_square[v] = true;
        }
    }
    let mut probable = false;
    for i in 0..=m {
        if is_sum_square[i] && is_sum_square[m - i] {
            probable = true;
            break;
        }
    }
    println!("{}", if probable { "Yes" } else { "No" });
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
