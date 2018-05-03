use std::collections::HashMap;

fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(&mut buf).ok();
    
    buf
}


fn fibo(n:i64, map:&mut HashMap<i64, i64>)->i64
{
    let mut r:i64 = 0;
    match map.get(&n)
    {
        Some(&number) => {
            number
        },
        _ => { 
            let num = fibo(n-1, map) + fibo(n-2, map);
            map.insert(n, num);
            num            
        },
    }
}

fn main() {
    let mut map:HashMap<i64, i64> = HashMap::new();

    let n = read_line().trim().parse::<i64>().unwrap();

    map.insert(1, 1);
    map.insert(2,1);

    let num = fibo(n+1, &mut map);

    println!("{}", num);

}
