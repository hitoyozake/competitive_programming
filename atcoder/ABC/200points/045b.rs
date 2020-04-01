use std::collections::HashMap;
use std::collections::VecDeque;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}

fn s_to_vec_deq(s: String)->VecDeque<char>{

    let mut vd: VecDeque<char> = VecDeque::new();

    for c in s.chars(){
        vd.push_back(c);
    }

    vd
}

fn main(){

    let a: String = read();let b: String = read();let c: String = read();
    
    let mut hashMap: HashMap<char, VecDeque<char>> = HashMap::new();
    
    hashMap.insert('a', s_to_vec_deq(a));
    hashMap.insert('b', s_to_vec_deq(b));
    hashMap.insert('c', s_to_vec_deq(c));
    
    let mut current = hashMap.get_mut(&'a').unwrap().pop_front().unwrap();
    
    while(true){
        let c = match hashMap.get_mut(& current).unwrap().pop_front(){
            Some(x) =>  x,
            _ => '\0'
        };

        if c == '\0' {
            break;
        }
        current = c;
    }

    current = match current{
        'a' => 'A',
        'b' => 'B',
        'c' => 'C',
        _ => '\0'
    };

    println!("{}", current);
    
}

