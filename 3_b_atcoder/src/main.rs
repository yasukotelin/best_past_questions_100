macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {
    let acgt = hashmap!('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0);
    let s: Vec<char> = read::<String>().chars().collect();

    let mut max_len = 0;
    let mut count = 0;
    for c in s.iter() {
        match acgt.get(c) {
            Some(_) => {
                count += 1;
            }
            None => {
                if max_len < count {
                    max_len = count;
                }
                count = 0;
            }
        }
    }
    if max_len < count {
        max_len = count;
    }

    println!("{}", max_len);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
