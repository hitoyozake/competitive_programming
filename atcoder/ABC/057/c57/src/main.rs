use std::cmp;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


fn get_digitlen(x: i64) -> usize{
    x.to_string().len()
}

fn f(a: i64, b: i64)->usize{
    cmp::max(get_digitlen(a), get_digitlen(b))
}

fn main() {

    let n = read::<String>().parse::<i64>().ok().unwrap();
    let mut i = 1;
    let mut result: usize = usize::MAX;
    while(i*i < n){
        let md = n%i;
        
        if md == 0 {
            result = cmp::min(result, f(i, n/i));
        }
        i += 1;
    }
    println!("{}", result);
}
