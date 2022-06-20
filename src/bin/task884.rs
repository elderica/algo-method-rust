fn main() {
    let nq = scan_line();
    let n: usize = nq[0];
    let q: usize = nq[1];

    let mut queue: Vec<i32> = vec![-1; n];
    let mut head = 0usize;
    let mut tail = 0usize;

    for _ in 0..q {
        let query: Vec<i32> = scan_line();
        match query[..] {
            [0, v] => {
                queue[tail] = v;
                tail = (tail + 1) % n;
            }
            [1] => {
                queue[head] = -1;
                head = (head + 1) % n;
            }
            _ => unreachable!(),
        }
    }

    // println!(
    //     "head: {} tail:{} queue len:{}",
    //     head,
    //     tail,
    //     if head <= tail {
    //         tail - head
    //     } else {
    //         (n - head) + tail
    //     }
    // );
    queue.into_iter().for_each(|e| println!("{}", e));
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
