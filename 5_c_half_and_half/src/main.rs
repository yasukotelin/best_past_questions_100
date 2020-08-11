fn main() {
    let input: Vec<i32> = read_vec();
    let a_price = input[0];
    let b_price = input[1];
    let ab_price = input[2];
    let a_num = input[3];
    let b_num = input[4];

    let mut min_price = 0;

    let n = std::cmp::max(a_num, b_num) + 1;
    for ab_count in 0..n {
        // ABは2倍の価格でAとB1枚ずつになる
        let mut price = ab_count * 2 * ab_price;
        if ab_count < a_num {
            // Aを買い足す
            price += (a_num - ab_count) * a_price;
        }
        if ab_count < b_num {
            // Bを買い足す
            price += (b_num - ab_count) * b_price;
        }

        if min_price == 0 {
            min_price = price;
        } else {
            min_price = std::cmp::min(min_price, price);
        }
    }

    println!("{}", min_price);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
