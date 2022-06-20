use std::collections::VecDeque;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let sluper = Slurper::stdin();
    #[allow(unused)]
    let _x: usize = sluper.scan_line()[0];
    let _q: usize = sluper.scan_line()[0];

    let mut scheduled_completion_times: VecDeque<usize> = VecDeque::new();
    let mut completed_task_counter: usize = 0;
    for _ in 0.._q {
        let op_id = sluper.read_word();
        let current_time: usize = sluper.read_word().parse().unwrap();
        match op_id {
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

use std::cell::Cell;
use std::io::{stdin, Read};

struct Slurper {
    str: String,
    pos: Cell<usize>,
}

impl Slurper {
    fn stdin() -> Self {
        let mut s = String::with_capacity(65536);
        stdin()
            .lock()
            .read_to_string(&mut s)
            .expect("read until EOF");
        Self {
            str: s,
            pos: Cell::new(0),
        }
    }

    fn read_line(&self) -> &str {
        let start_pos = self.pos.get();
        let mut current_pos = start_pos;
        while current_pos < self.str.len() && &self.str[current_pos..current_pos + 1] != "\n" {
            current_pos += 1;
        }
        self.pos.set(current_pos + 1);
        self.str[start_pos..current_pos].trim_end()
    }

    #[allow(dead_code)]
    fn read_word(&self) -> &str {
        let start_pos = self.pos.get();
        let mut current_pos = start_pos;
        while current_pos < self.str.len()
            && &self.str[current_pos..current_pos + 1] != " "
            && &self.str[current_pos..current_pos + 1] != "\n"
        {
            current_pos += 1;
        }
        self.pos.set(current_pos + 1);
        self.str[start_pos..current_pos].trim_end()
    }

    #[allow(dead_code)]
    fn read_words(&self) -> Vec<&str> {
        self.read_line().split_ascii_whitespace().collect()
    }

    #[allow(dead_code)]
    fn scan_line<F>(&self) -> Vec<F>
    where
        F: std::str::FromStr,
    {
        self.read_line()
            .split_ascii_whitespace()
            .flat_map(str::parse::<F>)
            .collect()
    }
}
