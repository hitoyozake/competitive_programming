
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

use core::cmp::max;
use core::cmp::min;

fn main() {
    let ms:String = read();
    let m:Vec<&str> = ms.split_whitespace().collect();
    let num = m[1].parse::<i32>().unwrap();

    let mut mx = 1;
    let mut mn = 10000000;

    for i in 0..num{
       let (a, b) = {
           let tmps:String = read();
           let mm:Vec<&str> = tmps.split_whitespace().collect();
            (mm[0].parse::<i32>().unwrap(), mm[1].parse::<i32>().unwrap()) 
       };

       mn = min(mn, b);
       mx = max(mx, a);
    }

    if mn-mx >= 0 {
    println!("{}", mn-mx+1);
    } else{
        println!("{}", 0);
    }

}
