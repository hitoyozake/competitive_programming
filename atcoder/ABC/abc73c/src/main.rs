use std::collections::HashMap;
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let num: String = read();
    let mut map: HashMap<i64> = HashMap::new();    
    for i in 0..num {

    }

    println!("Hello, world!");
}
