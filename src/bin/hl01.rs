use std::io::{BufRead, Result};

#[allow(arithmetic_overflow)]
fn main() -> Result<()> {
    let lock = std::io::stdin().lock();
    let mut buf = std::io::BufReader::new(lock);

    let mut sum: u64 = 0;
    let mut line = String::with_capacity(u16::MAX as usize);
    while buf.read_line(&mut line).unwrap() > 0 {
        let i = line.trim().parse::<u64>();
        if let Ok(i) = i {
            sum += i;
        }
        line.clear();
    }

    println!("{}", sum);

    Ok(())
}
