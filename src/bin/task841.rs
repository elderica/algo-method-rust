fn main() {
    let hw = scan_line();
    let h: usize = hw[0];
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let line = read_line().chars().collect();
        grid.push(line);
    }

    let largeq: usize = scan_line()[0];
    for _ in 0..largeq {
        let query = scan_line();
        match query[..] {
            [0usize, p, q] => {
                push(&mut grid, p, q);
            }
            [1, p, q] => {
                println!("{}", get(&grid, p, q));
            }
            _ => {}
        }
    }
}

#[allow(clippy::ptr_arg)]
fn push(grid: &mut Vec<Vec<char>>, p: usize, q: usize) {
    fn rev(c: char) -> char {
        if c == '#' {
            '.'
        } else {
            '#'
        }
    }
    // center
    grid[p][q] = rev(grid[p][q]);
    // down
    if p + 1 < grid.len() {
        grid[p + 1][q] = rev(grid[p + 1][q]);
    }
    // right
    if q + 1 < grid[p].len() {
        grid[p][q + 1] = rev(grid[p][q + 1]);
    }

    let p = p as isize;
    let q = q as isize;
    // left
    if p - 1 > -1 {
        let p = p as usize;
        let q = q as usize;
        grid[p - 1][q] = rev(grid[p - 1][q]);
    }
    // up
    if q - 1 > -1 {
        let p = p as usize;
        let q = q as usize;
        grid[p][q - 1] = rev(grid[p][q - 1]);
    }
}

fn get(grid: &[Vec<char>], p: usize, q: usize) -> usize {
    let mut count = 0;
    // center
    if grid[p][q] == '#' {
        count += 1
    }
    // down
    if p + 1 < grid.len() && grid[p + 1][q] == '#' {
        count += 1
    }
    // right
    if q + 1 < grid[p].len() && grid[p][q + 1] == '#' {
        count += 1;
    }

    let p = p as isize;
    let q = q as isize;
    // left
    if p - 1 > -1 {
        let p = p as usize;
        let q = q as usize;
        if grid[p - 1][q] == '#' {
            count += 1
        }
    }
    // up
    if q - 1 > -1 {
        let p = p as usize;
        let q = q as usize;
        if grid[p][q - 1] == '#' {
            count += 1;
        }
    }

    count
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
