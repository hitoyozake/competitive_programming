
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
    let n = read_line().trim().parse::<u32>().unwrap();

    let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();
    let mut map: std::collections::HashMap<u32,u32> = std::collections::HashMap::new();
    queue.push_front(1);
    let mut current:u32 = 1;
    let mut step:u32 = 0;

    map.insert(1, 1);

    while(queue.is_empty() == false){
        current = queue.pop_back().unwrap();
         match(map.get(&current)){
            Some(_found) =>{
                step = *_found;
            },
            _ => {
                // never reach
            },
        };
        if current == n{
            break;
        }

        let bitcount = get_bitcount(current);

        //get next
        let next = current + bitcount;
        let mut found = false;
        match(map.get(&next)){
            Some(_found) =>{
                found = true;
            },
            _ => {
                
            },
        };

        if found == false{
            map.insert(next, step+1);
                if next <= n {
                    queue.push_front(next);    
                }
        }

        let next = current - bitcount;
        let mut found = false;

        match(map.get(&next)){
            Some(_found) =>{
                found = true;
            },
            _ => {
            },
        };


        if found == false{
            map.insert(next, step+1);
                if next <= n {
                    queue.push_front(next);    
                }
        }
    }

    let mut r = -1;

    match(map.get(&n)){
        Some(_found)=>{
            r = *_found as i64;
        },
        _ =>{

        },
    };
    

    println!("{}", r);
}
