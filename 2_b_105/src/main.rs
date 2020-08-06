fn main() {
    let n: i32 = read();

    let mut total_count = 0;

    // 1以上N以下の整数
    for i in 1..(n + 1) {
        if i % 2 == 0 {
            // 偶数は対象外
            continue;
        }
        // 整数iの約数の個数を調べる
        let mut count = 0;
        for j in 1..(i + 1) {
            if i % j == 0 {
                count += 1;
            }
        }

        if count == 8 {
            total_count += 1;
        }
    }

    println!("{}", total_count);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

