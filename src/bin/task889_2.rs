use std::collections::VecDeque;
use std::io::Write;

fn main() {
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut sluper = Slurper::stdin();
    #[allow(unused)]
    let _x: usize = sluper.scan_line()[0];
    let _q: usize = sluper.scan_line()[0];

    let mut scheduled_completion_times: VecDeque<usize> = VecDeque::new();
    let mut completed_task_counter: usize = 0;
    for _ in 0.._q {
        let mut query = sluper.read_words().into_iter();
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
                writeln!(stdout, "{}", completed_task_counter).unwrap();
            }
            _ => unreachable!(),
        }
    }
}

use std::io::{stdin, stdout, BufRead, BufWriter, Cursor, Read};
struct Slurper {
    reader: Cursor<String>,
}

impl Slurper {
    fn stdin() -> Self {
        let mut s = String::with_capacity(1048576);
        stdin()
            .lock()
            .read_to_string(&mut s)
            .expect("read until EOF");
        let c = Cursor::new(s);
        Self { reader: c }
    }

    fn read_line(&mut self) -> String {
        let mut s = String::with_capacity(1024);
        self.reader.read_line(&mut s).unwrap();
        s.trim().to_string()
    }

    fn read_words(&mut self) -> Vec<String> {
        self.read_line()
            .split_ascii_whitespace()
            .map(String::from)
            .collect()
    }

    fn scan_line<F>(&mut self) -> Vec<F>
    where
        F: std::str::FromStr,
    {
        self.read_line()
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<F>())
            .collect()
    }
}
