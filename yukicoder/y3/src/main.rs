

fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(&mut buf);

    buf
}

fn main() {
    let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();

    let n = read_line().trim().parse::<u32>().unwrap();

    let s = String::from(read_line().trim());
    let v = s.split(" ");

    queue.push_back(v.front().parse::<u32>().unwrap());

    for i in v{
        let mut tmpv = std::collections::Vec::new();

        while(!queue.empty()){
            tmpv.push_back(queue.pop());
        }
    }
    
}
