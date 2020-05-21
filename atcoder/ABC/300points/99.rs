

fn read<T: std::str::FromStr>()->T{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}


fn main(){

    let s: String = read();

    let mut n: i64 = s.parse().ok().unwrap();

    let mut count = 0;

    while(n/9 > 0 && n%9 > 0){
        count += 1;
        n = n / 9 + n%9;
        println!("9");
    }
    println!("{}", n);
    while(n/6 > 0 && n%6 > 0){
        count += 1;
        n = n/6 + n%6;
        println!("6");
    }

    println!("{}", n);

    println!("{}", count + n);


}