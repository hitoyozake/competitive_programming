use std::collections::HashMap;

fn read<T: std::str::FromStr>()->T{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}

fn main(){
    let s: String = read();
    
    let (n, k) = {
        let mut ws = s.split_whitespace();
        let n:i64 = ws.next().unwrap().parse().unwrap();
        let k:i64 = ws.next().unwrap().parse().unwrap();
    
        (n, k)
    };


    let mut bucket: HashMap<i64, i64> = HashMap::new();

    for i in 0..n{
        let (b, times) = {
            let s:String = read();
            let mut ws = s.split_whitespace();
            let b:i64 = ws.next().unwrap().parse().unwrap();
            let times:i64 = ws.next().unwrap().parse().unwrap();
          
            (b, times)
        };

        //挿入
        let v = bucket.entry(b).or_insert(0);
        *v += times;
    }

    // sort
    let maxim = 100001;

    let mut index = 1;

    let mut mut_k = k;

    for i in 1..maxim{
        if mut_k <= 0 {
            break;
        }

        if let Some(x) = bucket.get(&i){
            index = i;
            mut_k -= *x;
        }
    }

    println!("{}", index);

}

