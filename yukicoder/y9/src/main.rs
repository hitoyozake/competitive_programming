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

//#[derive(Eq, Clone)]
#[derive(Eq)]
struct Mnt{
    level: i32,
    count: i32
}


// コピーを実装する
impl Clone for Mnt{
    fn clone(&self)->Mnt{
        Mnt{level:self.level, count:self.count}
    }
}

// 比較方法を実装する
impl PartialOrd for Mnt{
    fn partial_cmp(&self, other: &Mnt)->Option<Ordering>{
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


impl PartialEq for Mnt{
    fn eq(&self, other: &Mnt)->bool{
        (self.level, self.count) == (other.level, other.count)
    }

}

// BinaryHeapのために実装
impl Ord for Mnt{
    fn cmp(&self, other: &Mnt)->Ordering{
        
        (self.level, self.count).cmp(&(other.level, other.count))
        //(self.level, self.count).cmp(&(other.level, other.count))
    }
}

fn main() {

    struct Inches(i32);

    let length = Inches(10);

    let Inches(integer_length) = length;
    let il:Inches = Inches(20);

    let mut que:BinaryHeap<Mnt>= BinaryHeap::new();

    let n: i32 = get_line().trim().parse::<i32>().unwrap();

    let tmp = String::from(get_line().trim());
    let sv:Vec<&str> = tmp.split(" ").collect();

    let mut party: Vec<Mnt> = Vec::new();

    for i in sv{
        que.push(Mnt{ level:i.parse::<i32>().unwrap(), count: 0});
    }

    let tmp = String::from(get_line().trim());
    let sv:Vec<&str> = tmp.split(" ").collect();

    let mut el: Vec<i32> = Vec::new();

    for i in sv{
        el.push(i.parse::<i32>().unwrap()/2);
    }

    let mut max_count_min = 1000000;

    for i in 0..n{
        let mut pt = que.clone();

        let mut max_count = 0;
        
        for j in 0..n{
                let mut selected = pt.pop().unwrap();
                //println!("selected . l:{}, c:{}", selected.level, selected.count);
                selected.count+=1;

                let usz = ((i+j)%n) as u32;

                selected.level += el[usz as usize];
                max_count = std::cmp::max(max_count, selected.count);
                //println!("r . l:{}, c:{}", selected.level, selected.count);
        
                pt.push(selected);
        }
    
        max_count_min = std::cmp::min(max_count_min, max_count);

    }
    
    println!("{}", max_count_min);

}