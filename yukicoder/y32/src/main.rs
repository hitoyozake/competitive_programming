


fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(& mut buf).ok();

    buf
}



fn main() {
    let mut l = read_line().trim().parse::<i32>().unwrap();
    let mut m = read_line().trim().parse::<i32>().unwrap();
    let mut n = read_line().trim().parse::<i32>().unwrap();

    let x = l*100 + m*25 + n;

    let result = (x % 1000) / 100 + ((x % 1000) % 100) / 25 + (((x%1000)%100)%25); 
    

    println!("{}", result);
    
}
