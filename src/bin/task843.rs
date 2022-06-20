fn main() {
    let hw = scan_line();
    let h: usize = hw[0];
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let line = read_line().chars().collect();
        grid.push(line);
    }

    let mut nblacks = count(&grid);

    let largeq: usize = scan_line()[0];
    for _ in 0..largeq {
        let query = scan_line();
        match query[..] {
            [0usize, p, q] => {
                push(&mut grid, p, q, &mut nblacks);
            }
            [1] => {
                println!("{}", nblacks);
            }
            _ => {}
        }
    }
}

fn count(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    for l in grid.iter() {
        for c in l.iter() {
            if *c == '#' {
                count += 1
            }
        }
    }
    count
}

#[allow(clippy::ptr_arg)]
fn push(grid: &mut Vec<Vec<char>>, p: usize, q: usize, nblacks: &mut usize) {
    fn rev(c: &mut char, n: &mut usize) {
        if *c == '#' {
            *c = '.';
            *n -= 1;
        } else {
            *c = '#';
            *n += 1;
        }
    }
    // center
    rev(&mut grid[p][q], nblacks);
    // down
    if p + 1 < grid.len() {
        rev(&mut grid[p + 1][q], nblacks);
    }
    // right
    if q + 1 < grid[p].len() {
        rev(&mut grid[p][q + 1], nblacks);
    }

    let p = p as isize;
    let q = q as isize;
    // left
    if p - 1 > -1 {
        let p = p as usize;
        let q = q as usize;
        rev(&mut grid[p - 1][q], nblacks);
    }
    // up
    if q - 1 > -1 {
        let p = p as usize;
        let q = q as usize;
        rev(&mut grid[p][q - 1], nblacks);
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
