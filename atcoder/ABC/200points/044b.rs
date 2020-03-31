use std::collections::HashMap;

fn read<T: std::str::FromStr>()-> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}

fn main(){

    let s: String = read();

    let mut hashMap: HashMap<char, i32> = HashMap::new();

    for c in s.chars(){

        let count = hashMap.entry(c).or_insert(0);
        *count += 1;
    }

    let mut is_beautiful = true;

    for (c, v) in hashMap.iter() {
        if v % 2 == 1 {
            is_beautiful = false;
            break;
        }
    }

    if is_beautiful {
        println!("Yes")
    } 
    else {
        println!("No")
    }
}

