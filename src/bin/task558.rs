fn main() {
    let nm = scan_line();
    let n: usize = nm[0];
    let m: usize = nm[1];

    let mut uf = UnionFind::new(n);

    (0..m).for_each(|_| {
        let query: Vec<usize> = scan_line();
        if let [a, b] = *query {
            uf.unite(a, b);
        }
    });

    (0..n).for_each(|i| println!("{}", uf.min(i)));
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

#[allow(dead_code)]
fn read_words() -> Vec<String> {
    read_line()
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect()
}

#[allow(dead_code)]
fn scan_line<F>() -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line()
        .trim()
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}

pub const NIL: usize = usize::MAX;

pub struct UnionFind {
    pub parents: Vec<usize>,
    pub ranks: Vec<usize>,
    pub sizes: Vec<usize>,
    pub mins: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![NIL; n],
            ranks: vec![0; n],
            sizes: vec![1; n],
            mins: (0..n).collect(),
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == NIL {
            x
        } else {
            let r = self.root(self.parents[x]);
            self.parents[x] = r;
            r
        }
    }

    pub fn is_root(&mut self, x: usize) -> bool {
        self.root(x) == x
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.root(x), self.root(y));
        if rx == ry {
            return false;
        };
        if self.ranks[rx] < self.ranks[ry] {
            self.ranks.swap(rx, ry);
        };
        self.parents[ry] = rx;
        if self.ranks[rx] == self.ranks[ry] {
            self.ranks[rx] += 1;
        };
        self.sizes[rx] += self.sizes[ry];

        self.mins[rx] = std::cmp::min(self.mins[rx], self.mins[ry]);

        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.sizes[r]
    }

    pub fn min(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.mins[root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut uf = UnionFind::new(4);
        uf.unite(3, 2);
        assert_eq!(uf.size(3), 2);
        uf.unite(2, 0);
        assert_eq!(uf.size(0), 3);
    }

    #[test]
    fn smallest() {
        let n = 6;
        let mut uf = UnionFind::new(n);
        uf.unite(0, 1);
        uf.unite(2, 5);
        uf.unite(2, 3);

        let tests = (0..).zip([0, 0, 2, 2, 4, 2]);
        for (input, expected) in tests {
            assert_eq!(uf.min(input), expected);
        }
    }
}
