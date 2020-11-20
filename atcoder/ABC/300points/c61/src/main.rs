use std::collections::BTreeMap;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


fn main() {
    let (n, k) = {
        let s = read::<String>();
        let s2:Vec<&str> = s.split(' ').collect();
        let mut iter = s2.iter();
        (iter.next().unwrap().parse::<i64>().unwrap(), iter.next().unwrap().parse::<i64>().unwrap())
    };

    let mut hash:BTreeMap<i64, i64> = BTreeMap::new();

    for i in 0..n {
        let (x, y) = {
            let s = read::<String>();
            let s2:Vec<&str> = s.split(' ').collect();
            let mut iter = s2.iter();
            (iter.next().unwrap().parse::<i64>().unwrap(), iter.next().unwrap().parse::<i64>().unwrap())
        };
        let m = hash.entry(x).or_insert(0);
        *m += y;
    }

    let mut counter = 0;

    //dbg!(n, k);

    for (key, value) in hash{
        
        //dbg!(num);
        counter += value;

        if counter >= k{

            println!("{}", key);
            break;
        }
        //dbg!(counter);
    }
}
