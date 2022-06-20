use std::io::{stdout, BufWriter, Result, Write};

fn main() -> Result<()> {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let route = Problem::input().solve();
    writeln!(out, "{}", format_slice(&route))?;

    Ok(())
}

fn format_slice(v: &[usize]) -> String {
    let l = v
        .iter()
        .map(usize::to_string)
        .collect::<Vec<String>>()
        .join(" ");
    l
}

struct Problem {
    n: usize,
    ps: Vec<usize>,
}

impl Problem {
    fn input() -> Self {
        let sluper = Slurper::stdin();

        let n: usize = sluper.scan_line()[0];
        let ps: Vec<usize> = sluper.scan_line();

        Self { n, ps }
    }

    fn solve(&self) -> Vec<usize> {
        let n = self.n;
        let ps = &self.ps;

        let mut parent_of: Vec<usize> = vec![0; n];
        parent_of[1..].copy_from_slice(ps);
        let mut childs_of: Vec<Vec<usize>> = vec![vec![]; n];
        #[allow(clippy::needless_range_loop)]
        for i in 1..n {
            let p = parent_of[i];
            childs_of[p].push(i);
        }

        let mut route: Vec<usize> = Vec::new();
        rec(&childs_of, &mut route, 0);

        route
    }
}

#[allow(clippy::ptr_arg)]
fn rec(childs_of: &Vec<Vec<usize>>, route: &mut Vec<usize>, v: usize) {
    route.push(v);
    for v in childs_of[v].iter() {
        rec(childs_of, route, *v);
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
