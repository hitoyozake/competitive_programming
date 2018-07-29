
fn read_i64()->i64{

    let mut buf = String::new();

    let x = std::io::stdin().read_line(& mut buf);

    buf.trim().parse::<i64>().unwrap()

}



fn main() {
    let n = read_i64();

    let mut max:i64 = 0;

    for i in 0..n{

        max = std::cmp::max(max, (n-i+ 1)*i + (n-i));
    
    }

    println!("{}", max);

}
