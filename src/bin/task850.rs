fn main() {
    let n = scan_line();
    let n: usize = n[0];

    let mut mmkdt: Vec<usize> = (1..=n).collect();
    let mut n = n;
    let mut i = 0;
    let mut is_removable = true;
    while n != 1 {
        if mmkdt[i] == 0 {
            while mmkdt[i] == 0 {
                i = (i + 1) % mmkdt.len();
            }
        } else if is_removable {
            mmkdt[i] = 0;
            n -= 1;
            is_removable = false;
            i = (i + 1) % mmkdt.len();
        } else {
            is_removable = true;
            i = (i + 1) % mmkdt.len();
        }
        // println!("{:?}", mmkdt);
    }
    println!("{}", mmkdt.iter().sum::<usize>());
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

#[allow(dead_code)]
fn read_words() -> Vec<String> {
    read_line().split_whitespace().map(String::from).collect()
}

#[allow(dead_code)]
fn scan_line<F>() -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line()
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}
