
fn read<T: std::str::FromStr>()->T{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}


fn main(){

    let (n, y) = {
        let s: String  = read();
        let mut ws = s.split_whitespace();
        let n:i64 = ws.next().unwrap().parse().unwrap();
        let k:i64 = ws.next().unwrap().parse().unwrap();
    
        (n, k)
    };

    let mut find = false;

    for i in 0..=n{
        for j in 0..=n-i{
            let k = n - i - j;
            if i*10000 + j*5000 + k*1000 == y {
                find = true;
                println!("{} {} {}", i, j, k);
                break;
            }
        }

        if find == true{
            break;
        }
    }

    if find == false {
        println!("-1 -1 -1");
    }
}


