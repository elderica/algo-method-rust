fn main() {
    #[allow(unused)]
    let n: usize = scan_line()[0];
    let _a: Vec<usize> = scan_line();
    let mut worse_list: Vec<(usize, usize)> = Vec::with_capacity(n);

    worse_list.push((0, 0));

    for (i, a) in _a.iter().enumerate().skip(1) {
        while let Some(&(score, _)) = worse_list.last() {
            if *a > score {
                break;
            }
            worse_list.pop();
        }
        if let Some(&(_, day)) = worse_list.last() {
            let answer = i - day;
            println!("{}", answer);
            worse_list.push((*a, i));
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
