fn main() {
    let n: Vec<usize> = scan_line();
    let n = n[0];

    let 初期の並び: Vec<usize> = scan_line();

    let 無人 = usize::MAX;
    let mut の後ろ: Vec<usize> = vec![無人; n];
    let mut の前: Vec<usize> = vec![無人; n];

    for (&v, i) in 初期の並び.iter().zip(0usize..) {
        // 前に誰もいないとき
        if i != 0 {
            の前[v] = 初期の並び[i - 1];
        }
        if i != 初期の並び.len() - 1 {
            の後ろ[v] = 初期の並び[i + 1];
        }
    }

    let q: Vec<usize> = scan_line();
    let q = q[0];

    for _ in 0..q {
        let query: Vec<usize> = scan_line();

        match *query {
            [0, ランナーv] => {
                let 先行ランナー = の前[ランナーv];

                if 先行ランナー == 無人 {
                    println!("Error");
                    continue;
                }

                println!("{}", 先行ランナー);

                if の前[先行ランナー] != 無人 {
                    の後ろ[の前[先行ランナー]] = ランナーv;
                }
                if の後ろ[ランナーv] != 無人 {
                    の前[の後ろ[ランナーv]] = 先行ランナー;
                }
                の前[ランナーv] = の前[先行ランナー];
                の後ろ[先行ランナー] = の後ろ[ランナーv];
                の後ろ[ランナーv] = 先行ランナー;
                の前[先行ランナー] = ランナーv;
            }
            [1, 棄権者] => {
                if の前[棄権者] != 無人 {
                    の後ろ[の前[棄権者]] = の後ろ[棄権者];
                }
                if の後ろ[棄権者] != 無人 {
                    の前[の後ろ[棄権者]] = の前[棄権者];
                }
                の前[棄権者] = 無人;
                の後ろ[棄権者] = 無人;
            }
            _ => {}
        }
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
