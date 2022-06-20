use std::collections::VecDeque;

fn main() {
    #[allow(unused)]
    let _x: usize = scan_line()[0];
    let _q: usize = scan_line()[0];

    let mut scheduled_completion_times: VecDeque<usize> = VecDeque::new();
    let mut completed_task_counter: usize = 0;
    for _ in 0.._q {
        let mut query = read_words().into_iter();
        let op_id = query.next().expect("query has operation id");
        let current_time: usize = query.next().unwrap().parse().unwrap();
        match op_id.as_str() {
            "0" => {
                scheduled_completion_times.push_back(current_time + _x);
            }
            "1" => {
                while let Some(&s) = scheduled_completion_times.front() {
                    if s <= current_time {
                        completed_task_counter += 1;
                        scheduled_completion_times.pop_front();
                    } else {
                        break;
                    }
                }
                println!("{}", completed_task_counter);
            }
            _ => unreachable!(),
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
