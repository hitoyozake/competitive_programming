


fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(& mut buf).ok();

    buf
}



fn main() {
    let mut n = read_line().trim().parse::<i32>().unwrap();
    let mut m = read_line().trim().parse::<i32>().unwrap();
    let mut l = read_line().trim().parse::<i32>().unwrap();

    m = m + l / 25; // 25円玉に1円玉を両替した結果を反映
    l = l % 25; // 両替したのでその分減らす(あまり)
    n = n + m/4; // 25*4 -> 100円にする
    m = m % 4;
    n = n % 10;
    //println!("l:{}, m:{}, n:{}", l, m, n);

    println!("{}", l + m + n);
    
}
