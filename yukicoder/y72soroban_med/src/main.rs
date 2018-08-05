
fn read_i64()->i64{

    let mut buf = String::new();

    let x = std::io::stdin().read_line(& mut buf);

    buf.trim().parse::<i64>().unwrap()

}



fn main() {
    let n = read_i64() as i128;

    let mut max:i128 = 0;
    let nmod:i128 = n;

    for i in nmod/2-100..nmod/2+100{
        //println!("{}", (n-i+ 1)*i + (n-i));
        
        let imod:i128 = i;
        
        max = std::cmp::max(max, ((((nmod-imod+1))*(imod)) + (nmod-imod))%1000007);
    
    }

    println!("{}", max);

}
