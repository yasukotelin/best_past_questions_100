fn main() {
    let mut inputs: Vec<(i32, i32)> = Vec::new();
    loop {
        let input: Vec<i32> = read_vec();
        if input[0] == 0 && input[1] == 0 {
            break;
        }
        inputs.push((input[0], input[1]));
    }

    let mut counts: Vec<i32> = Vec::new();

    for input in inputs.iter() {
        let n = input.0 + 1;
        let x = input.1;

        let mut count = 0;

        // 重複なく3つの数値を全探索する
        for i in 1..(n - 2) {
            for j in (i + 1)..(n - 1) {
                for k in (j + 1)..n {
                    if x == i + j + k {
                        count += 1;
                        break;
                    }
                }
            }
        }

        counts.push(count);
    }

    for r in counts.iter() {
        println!("{}", r);
    }
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
