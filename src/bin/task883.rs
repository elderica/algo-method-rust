use std::collections::VecDeque;

fn main() {
    let _x = read_line();
    let _n: isize = scan_line()[0];
    let _s = read_words();

    let mut stack: VecDeque<isize> = VecDeque::new();
    for token in _s {
        match token.as_str() {
            "+" => {
                let operand2 = stack.pop_back().expect("operand1 must be available");
                let operand1 = stack.pop_back().expect("operand2 must be available");
                stack.push_back(operand1 + operand2);
            }
            "-" => {
                let operand2 = stack.pop_back().expect("operand1 must be available");
                let operand1 = stack.pop_back().expect("operand2 must be available");
                stack.push_back(operand1 - operand2);
            }
            "*" => {
                let operand2 = stack.pop_back().expect("operand1 must be available");
                let operand1 = stack.pop_back().expect("operand2 must be available");
                stack.push_back(operand1 * operand2);
            }
            str_num => {
                let num = str_num.parse().expect("not numeric token");
                stack.push_back(num);
            }
        }
    }

    let answer = stack.pop_back().expect("answer must be available");
    println!("{}={}", _x, answer);
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
