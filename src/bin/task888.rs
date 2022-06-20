fn main() {
    #[allow(unused)]
    let n: usize = scan_line()[0];
    let s = read_line();
    let mut counter = 0;

    for c in s.chars() {
        match c {
            '(' => counter += 1,
            ')' => {
                counter -= 1;
                if counter < 0 {
                    break;
                }
            }
            _ => {}
        }
    }

    if counter == 0 {
        println!("Yes");
    } else {
        println!("No");
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
