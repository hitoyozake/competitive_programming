use std::collections::HashSet;
use std::cmp;

fn read<T: std::str::FromStr>()->T{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}

fn main(){

        //nの読み込み
        let n: i64 = {

            let s: String = read();
            
            s.parse().unwrap()
        };

        let mut maxcount:usize = 0;
        let mut color_set = HashSet::new();

        //n使わない
        let s: String = read();
        let mut smut = s.split_whitespace();

        //

        for i in smut{
            let a: i64 = i.parse().unwrap();
            
            let col = match a {
                1...399 =>0,
                400...799 =>1,
                800...1199 =>2,
                1200...1599 =>3,
                1600...1999 =>4,
                2000...2399 =>5,
                2400...2799 =>6,
                2800...3199 =>7,
                _ => 8
            };

            if col == 8 {
                maxcount += 1;
            }
            else{            
                color_set.insert(col);
            }
        }

        let COLORMAX:usize = 8;
        let MIN_COLOR:usize = 1;

        let mut min_color = cmp::max( MIN_COLOR,  color_set.len());
        let mut max_color = cmp::max( MIN_COLOR,  color_set.len() + maxcount);

        println!("{} {}", min_color, max_color);


}
