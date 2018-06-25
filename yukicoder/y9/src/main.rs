use std::collections::BinaryHeap;

struct Unit{
    level: i32,
    count: i32
}

fn get_line()->String{
    let mut buf = String::new();

    let r = std::io::stdin().read_line(& mut buf);

    buf
}

use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd)]
struct Monster{
    level: i32,
    count: i32
}

// BinaryHeapのために実装
impl Ord for Monster{
    fn cmp(&self, other: &Monster)->Ordering{
        (self.level, self.count).cmp(&(other.level, other.count))
    }

}


fn main() {

    struct Inches(i32);

    let length = Inches(10);

    let Inches(integer_length) = length;
    let il:Inches = Inches(20);

    let mut heap:BinaryHeap<i32>= BinaryHeap::new();

    let n: i32 = get_line().trim().parse::<i32>().unwrap();

    let tmp = String::from(get_line().trim());
    let sv:Vec<&str> = tmp.split(",").collect();

    let mut party: Vec<Monster> = Vec::new();

    for i in sv{
        party.push(Monster{ level:i.parse::<i32>().unwrap(), count: 0});
    }

    let tmp = String::from(get_line().trim());
    let sv:Vec<&str> = tmp.split(",").collect();

    let mut enem_levels: Vec<i32> = Vec::new();

    for i in sv{
        enem_levels.push(i.parse::<i32>().unwrap()/2);
    }

    let mut max_count_min = 1000000;

    for i in 0..n{
        let pt = party.Clone();

    }
    

}