fn main() {
    let hw = scan_line();
    let h: usize = hw[0];
    let w: usize = hw[1];
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let line = read_line().chars().collect();
        grid.push(line);
    }

    let mut linecounts = vec![];
    #[allow(clippy::needless_range_loop)]
    for l in 0..h {
        let count = grid[l].iter().filter(|&&ch| ch == '#').count();
        linecounts.push(count);
    }
    let mut columncounts = vec![];
    for c in 0..w {
        let mut count = 0;
        #[allow(clippy::needless_range_loop)]
        for l in 0..h {
            if grid[l][c] == '#' {
                count += 1;
            }
        }
        columncounts.push(count);
    }

    let largeq: usize = scan_line()[0];
    for _ in 0..largeq {
        let pq = scan_line();
        let p: usize = pq[0];
        let q: usize = pq[1];

        println!(
            "{}",
            if grid[p as usize][q as usize] == '#' {
                linecounts[p] + columncounts[q] - 1
            } else {
                linecounts[p] + columncounts[q]
            }
        );
    }
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
