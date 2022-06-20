use std::io::{stdout, BufWriter, Write};

fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let sluper = Slurper::stdin();

    let n = sluper.scan_line::<usize>()[0];
    let mut parent_of: Vec<usize> = vec![0; n];
    let mut childs_of: Vec<Vec<usize>> = vec![vec![]; n];
    for _i in 0..=(n - 2) {
        let ab = sluper.scan_line::<usize>();
        let (a, b) = (ab[0], ab[1]);
        parent_of[b] = a;
        childs_of[a].push(b);
    }
    let q = sluper.scan_line::<usize>()[0];
    for _ in 0..q {
        let v = sluper.scan_line::<usize>()[0];
        let childs = &mut childs_of[parent_of[v]];
        childs.sort_unstable();
        writeln!(out, "{}", format_slice(childs)).unwrap();
    }
}

fn format_slice(v: &[usize]) -> String {
    let l = v
        .iter()
        .map(usize::to_string)
        .collect::<Vec<String>>()
        .join(" ");
    l
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
