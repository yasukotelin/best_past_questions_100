fn main() {
    let n: i32 = read();
    let mut positions: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        let input: Vec<i32> = read_vec();
        positions.push((input[0], input[1]));
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
