use std::collections::HashMap;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


fn main() {
    let s1: Vec<char>  = read::<String>().chars().collect();
    let s2: Vec<char> = read::<String>().chars().collect();

    let mut dict1: HashMap<char, char>  = HashMap::new();
    let mut dict2: HashMap<char, char>  = HashMap::new();

    //&char, &char 
    for (i, j) in s1.iter().zip(s2.iter()) {
        match dict1.get(i){
            Some(v) => {
                if v != j{
                    println!("No");
                    return;
                }
            
            } ,
            None => {
                // 値を入れる
                dict1.insert(*i, *j);
            }
        }
        match dict2.get(j){
            Some(v) => {
                if v != i{
                    println!("No");
                    return;
                }
            },
            None => {
                // 値を入れる
                dict2.insert(*j, *i);
            }
        }
    }

    println!("Yes");

}
