
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main(){
    let s: String = read();
    let mut s2 = s.split(" ");

    let (a, b) = {
        (s2.next().unwrap().parse::<i32>().unwrap(), s2.next().unwrap().parse::<i32>().unwrap())
    };
    let mut count = 0;
    for i in 0..=a{
        for j in 0..=a{
            if b-i-j <= a && b-i-j >= 0{
                count+=1;
            }
        }
    }
    println!("{}", count);
}




