use std::collections::HashMap;

fn main() {
    let m: u32 = read();
    let constellation: Vec<Vec<i32>> = read_vec2(m);
    let n: u32 = read();
    let picture: Vec<Vec<i32>> = read_vec2(n);

    // 星座の位置関係
    let constellation_base = &constellation[0];
    let mut pos_cordinate: Vec<(i32, i32)> = Vec::new();
    for i in 1..m {
        let p_x = constellation[i as usize][0];
        let p_y = constellation[i as usize][1];

        pos_cordinate.push((constellation_base[0] - p_x, constellation_base[1] - p_y));
    }

    // 写真の座標をHashMap化
    let mut picture_hash_map: HashMap<(i32, i32), bool> = HashMap::new();
    for i in 0..n {
        picture_hash_map.insert((picture[i as usize][0], picture[i as usize][1]), true);
    }

    // 座標関係から写真座標に一致する星座を探索する
    for p in picture.iter() {
        if is_include(&(p[0], p[1]), &picture_hash_map, &pos_cordinate) {
            // 座標に一致する星座を見つけたとき
            // pとconstellation_baseの星座の位置関係は同じなので、差が座標の移動量になる
            let x = p[0] - constellation_base[0];
            let y = p[1] - constellation_base[1];
            println!("{} {}", x, y);
            return;
        }
    }
}

fn is_include(
    base: &(i32, i32),
    picture_hash_map: &HashMap<(i32, i32), bool>,
    cordinate: &Vec<(i32, i32)>,
) -> bool {
    for cordinate in cordinate.iter() {
        let target = (base.0 - cordinate.0, base.1 - cordinate.1);
        if let None = picture_hash_map.get(&target) {
            return false;
        }
    }

    return true;
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
