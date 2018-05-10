
fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(&mut buf);

    return buf;
}

fn get_bitcount(x:u32)->u32{
    let mut tmp = x;
    let mut count = 0;
    for i in 0..16{
        count += tmp % 2;

        tmp = tmp >> 1;
    }
    count
}



fn main() {
    
    for i in 0..32{
        println!("{}:{}", i, get_bitcount(i));
    }

    println!("Hello, world!");
}
