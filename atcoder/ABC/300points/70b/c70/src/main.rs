
use std::cmp;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


fn main() {
    let (a, b, c, d) = {
        let s2:String = read();
        let s = s2.trim();
        let mut v:Vec<&str> = s.split(' ').collect();
        let mut iter = v.iter();
        (iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap())
    };

    let common = cmp::max(0, cmp::min(b, d) - cmp::max(a, c));
    println!("{}", common);


}
