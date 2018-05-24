
fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(&mut buf);

    buf
}


fn main() {

    let max:i32 = read_line().trim().parse::<i32>().unwrap();

    let n:i32 = read_line().trim().parse::<i32>().unwrap();

    let blocks = String::from(read_line().trim());

    let v = blocks.split(" ");

    let num_vect: Vec<i32> = Vec::new();


    println!("Hello, world!");
}
