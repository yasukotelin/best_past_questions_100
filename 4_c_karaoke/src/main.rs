fn main() {
    let input: Vec<i32> = read_vec();
    let students_number: i32 = input[0];
    let music_number: i32 = input[1];
    let students_scores: Vec<Vec<i32>> = read_vec2(students_number);

    let mut max_score: i64 = 0;

    // 曲を2曲全選択するforネスト
    for m1 in 0..(music_number - 1) {
        for m2 in 1..music_number {
            // 選択した2曲のグループ得点を算出
            let mut total_score: i64 = 0;
            for scores in students_scores.iter() {
                let score_m1 = scores[m1 as usize];
                let socre_m2 = scores[m2 as usize];
                let score = get_max(score_m1, socre_m2);

                total_score += score as i64;
            }

            if total_score > max_score {
                // 最大点を更新
                max_score = total_score;
            }
        }
    }

    println!("{}", max_score);
}

fn get_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
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

fn read_vec2<T: std::str::FromStr>(n: i32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
