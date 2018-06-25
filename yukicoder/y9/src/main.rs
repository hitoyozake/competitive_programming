use std::collections::BinaryHeap;

struct Unit{
    level: i32,
    count: i32
}

fn get_line()->String{
    let mut buf = String::new();

    let r = std::io::stdin().get_line(mut & buf);

    buf
}


fn main() {

    struct Inches(i32);

    let length = Inches(10);

    let Inches(integer_length) = length;
    let il:Inches = Inches(20);

    let mut heap:BinaryHeap<i32>= BinaryHeap::new();




    println!("Hello, world!");
}