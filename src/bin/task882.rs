use std::collections::VecDeque;

fn main() {
    let _q = scan_line()[0];

    let mut queue: VecDeque<String> = VecDeque::new();
    for _ in 0.._q {
        let mut query = read_words().into_iter();
        let qtype = query
            .next()
            .expect("first element of query must be available");
        match qtype.as_str() {
            "0" => {
                let qtask = query.next().expect("Set query must has a task");
                queue.push_back(qtask)
            }
            "1" => {
                let task = queue.pop_front().expect("queue must not be empty");
                println!("{}", task);
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
