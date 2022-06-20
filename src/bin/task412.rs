use std::io::{stdout, BufWriter, Write};

fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let sluper = Slurper::stdin();

    if let [n, m] = sluper.scan_line::<usize>()[..] {
        let mut follows: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
        for _ in 0..m {
            if let [i, j] = sluper.scan_line::<usize>()[..] {
                follows[i].push(j);
            }
        }

        for f in &mut follows {
            f.sort_unstable();
            let l = f
                .iter()
                .map(usize::to_string)
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(out, "{}", l).unwrap();
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
