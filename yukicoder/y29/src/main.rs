
fn get_line()->String{
    let mut buf = String::new();
    let x = std::io::stdin().read_line(& mut buf);
    buf
}

use std::collections::HashMap;

fn main() {

    let n = get_line().trim().parse::<u64>().unwrap();
    let mut hashMap: HashMap<u64, i64> = HashMap::new();
    for i in 0..n{
        let s = get_line();
        let items = s.trim().split(' ');

        for item in items{
            let i:u64 = item.parse::<u64>().unwrap();
            *hashMap.entry(i).or_insert(0) += 1;
        }
    }
}
