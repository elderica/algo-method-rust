use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let sluper = Slurper::stdin();

    if let [n, m, x] = sluper.scan_line::<usize>()[..] {
        let mut friendss: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for _ in 0..m {
            if let [i, j] = sluper.scan_line::<usize>()[..] {
                friendss[i].insert(j);
                friendss[j].insert(i);
            }
        }
        let my_friend = &friendss[x];
        let mut result: HashSet<usize> = HashSet::new();
        for friend in friendss[x].iter() {
            let friends_of_friend = &friendss[*friend];
            for friend_of_friend in friends_of_friend {
                if *friend_of_friend != x && !my_friend.contains(friend_of_friend) {
                    result.insert(*friend_of_friend);
                }
            }
        }
        writeln!(out, "{}", result.len()).unwrap();
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
