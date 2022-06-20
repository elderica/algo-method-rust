use std::{
    collections::HashSet,
    io::{BufRead, Result},
};

#[allow(arithmetic_overflow)]
fn main() -> Result<()> {
    let lock = std::io::stdin().lock();
    let mut buf = std::io::BufReader::new(lock);

    let mut hs = HashSet::new();
    let mut line = String::with_capacity(20);
    while buf.read_line(&mut line).unwrap() > 0 {
        hs.insert(line.to_owned());
        line.clear();
    }

    println!("{}", hs.len());

    Ok(())
}
