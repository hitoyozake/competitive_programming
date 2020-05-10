use std::collections::HashMap;

fn read<T: std::str::FromStr>()->T{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}


fn main(){
    
    let s: String = read();
    
    let n:i64 = s.parse().unwrap();

    let mut vec: Vec<i64> = Vec::new();

    
    let mut mp:HashMap<i64, i64> = HashMap::new();

    for i in 0..n{
        let tmp: String = read();
        let num: i64 = tmp.parse().ok().unwrap();
        
        let c = mp.entry(num).or_insert(0);
        *c += 1;
    }

  

    println!("{}", mp.iter().filter(|&(k, v)| *v%2==1).count());

}



