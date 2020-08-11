fn main() {
    let n: i32 = read();
    let chars: Vec<char> = read::<String>().chars().collect();

    let mut count = 0;

    // 000 ~ 999の暗証番号を作成できるかを探索
    for i in 0..1000 {
        let pass = format!("{:0width$}", i, width = 3)
            .chars()
            .collect::<Vec<_>>();

        if can_create(n, &chars, &pass) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn can_create(n: i32, chars: &Vec<char>, pass: &Vec<char>) -> bool {
    // 1文字目があるか
    for j in 0..(n - 2) {
        if chars[j as usize] != pass[0] {
            continue;
        }

        // 1文字目があった場合のみ2文字目探索
        for k in (j + 1)..(n - 1) {
            if chars[k as usize] != pass[1] {
                continue;
            }

            // 2文字目があった場合のみ3文字目探索
            for l in (k + 1)..n {
                if chars[l as usize] != pass[2] {
                    continue;
                }
                return true;
            }

            // 3文字目が見つからない場合は即終了
            return false;
        }

        // 2文字目が見つからない場合は即終了
        return false;
    }

    // 1文字目が見つからない場合
    return false;
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
