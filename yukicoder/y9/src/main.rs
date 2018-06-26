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

#[derive(Eq)]
struct Monster{
    level: i32,
    count: i32
}


impl Clone for Monster{
    fn clone(&self)->Monster{
        Monster{level:self.level, count:self.count}
    }
}

impl PartialOrd for Monster{
    fn partial_cmp(&self, other: &Monster)->Option<Ordering>{
        if self.level < other.level
        {
            Some(Ordering::Greater)
        }
        else if self.level == other.level && self.count <= other.count {
                Some(Ordering::Greater)
        }
        else{
            Some(Ordering::Less)
        }
    }
}


impl PartialEq for Monster{
    fn eq(&self, other: &Monster)->bool{
        (self.level, self.count) == (other.level, other.count)
    }

}

// BinaryHeapのために実装
impl Ord for Monster{
    fn cmp(&self, other: &Monster)->Ordering{
        
        (self.level, self.count).cmp(&(other.level, other.count))
        //(self.level, self.count).cmp(&(other.level, other.count))
    }
}

fn main() {

    struct Inches(i32);

    let length = Inches(10);

    let Inches(integer_length) = length;
    let il:Inches = Inches(20);

    let mut que:BinaryHeap<Monster>= BinaryHeap::new();

    let n: i32 = get_line().trim().parse::<i32>().unwrap();

    let tmp = String::from(get_line().trim());
    let sv:Vec<&str> = tmp.split(" ").collect();

    let mut party: Vec<Monster> = Vec::new();

    for i in sv{
        que.push(Monster{ level:i.parse::<i32>().unwrap(), count: 0});
    }

    let tmp = String::from(get_line().trim());
    let sv:Vec<&str> = tmp.split(" ").collect();

    let mut enem_levels: Vec<i32> = Vec::new();

    for i in sv{
        enem_levels.push(i.parse::<i32>().unwrap()/2);
    }

    let mut max_count_min = 1000000;

    for i in 0..n{
        let mut pt = que.clone();

        let mut max_count = 0;
        //popされた時の値をうまくなるように調整が必要そう
        for j in 0..n{
                let mut selected = pt.pop().unwrap();
                //println!("selected . l:{}, c:{}", selected.level, selected.count);
                selected.count+=1;

                let usz = ((i+j)%n) as u32;

                selected.level += enem_levels[usz as usize];
                max_count = std::cmp::max(max_count, selected.count);
                //println!("r . l:{}, c:{}", selected.level, selected.count);
        
                pt.push(selected);
        }
    
        max_count_min = std::cmp::min(max_count_min, max_count);

    }
    
    println!("{}", max_count_min);

}