use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, StdinLock, Write};

fn main() {
    let stdin = stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    #[allow(unused)]
    let _x: usize = scan_line(&mut stdin)[0];
    let _q: usize = scan_line(&mut stdin)[0];

    let mut scheduled_completion_times: VecDeque<usize> = VecDeque::new();
    let mut completed_task_counter: usize = 0;
    for _ in 0.._q {
        let mut query = read_words(&mut stdin).into_iter();
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
                writeln!(stdout, "{}", completed_task_counter).ok();
            }
            _ => unreachable!(),
        }
    }
}

fn read_line(r: &mut BufReader<StdinLock>) -> String {
    let mut line = String::new();
    r.read_line(&mut line).ok();
    line.trim().to_string()
}

#[allow(dead_code)]
fn read_words(r: &mut BufReader<StdinLock>) -> Vec<String> {
    read_line(r).split_whitespace().map(String::from).collect()
}

#[allow(dead_code)]
fn scan_line<F>(r: &mut BufReader<StdinLock>) -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line(r)
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}
