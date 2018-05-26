use std::collections::LinkedList;

fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(& mut buf);

    buf
}

fn main() {
    let mut list = Vec::new();
    let mut primes = Vec::new();
    let max_n:f32 = 200000.0 + 1.0;
    let sqrt_num = max_n.sqrt().abs() as i32;
    for i in 2..20000{
        list.push(i);
    }

    
    while !list.is_empty(){
        let mut tmp = Vec::new();
        let front = list[0];
        primes.push(front);

        if front >= sqrt_num{
            break;
        }

        for i in list{
            if i%front!=0{
                tmp.push(i);
            }
        }
        list = tmp;
    }

    primes.append(&mut list);

    

}
