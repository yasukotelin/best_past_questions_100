fn main() {
    let n: u32 = read();
    let inputs: Vec<Vec<i64>> = read_vec2(n);

    let mut min_total = 0;
    for i in 0..n {
        // 入口と出口はinputsのなかのいずれかになる様子
        // 2重ループでstartとendの組み合わせで全探索する
        let start = inputs[i as usize][0];

        for j in 0..n {
            let end = inputs[j as usize][1];

            let mut total = 0;

            for input in inputs.iter() {
                let m1 = input[0];
                let m2 = input[1];

                total += (start - m1).abs() + (m1 - m2).abs() + (m2 - end).abs();
            }

            if i == 0 && j == 0 {
                min_total = total;
            } else {
                if min_total > total {
                    min_total = total;
                }
            }
        }
    }

    println!("{}", min_total);
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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
