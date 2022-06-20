fn main() {
    let hw = scan_line();
    let h: u16 = hw[0];
    let w: u16 = hw[1];
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let line = read_line().chars().collect();
        grid.push(line);
    }
    let pq = scan_line();
    let p: u16 = pq[0];
    let q: u16 = pq[1];

    // println!("h:{} w:{}, p:{}, q:{}", h, w, p, q);
    // println!("{:#?}", grid);

    let mut count = 0;
    for c in 0..w {
        if grid[p as usize][c as usize] == '#' {
            count += 1;
        }
    }
    for l in 0..h {
        if grid[l as usize][q as usize] == '#' {
            count += 1;
        }
    }
    println!(
        "{}",
        if grid[p as usize][q as usize] == '#' {
            count - 1
        } else {
            count
        }
    );
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
