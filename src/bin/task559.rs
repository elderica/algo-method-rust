fn main() {
    let nm = scan_line();
    let n: usize = nm[0];
    let m: usize = nm[1];

    let mut uf = UnionFind::new(n);

    (0..m).for_each(|_| {
        let query: Vec<usize> = scan_line();
        if let [a, b] = *query {
            if uf.is_same(a, b) {
                println!("Yes");
            } else {
                println!("No");
                uf.unite(a, b);
            }
        }
    });
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
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
    pub siz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![NIL; n],
            rank: vec![0; n],
            siz: vec![1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == NIL {
            x
        } else {
            let r = self.root(self.par[x]);
            self.par[x] = r;
            r
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.root(x), self.root(y));
        if rx == ry {
            return false;
        };
        if self.rank[rx] < self.rank[ry] {
            self.rank.swap(rx, ry);
        };
        self.par[ry] = rx;
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        };
        self.siz[rx] += self.siz[ry];

        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.siz[r]
    }
}
