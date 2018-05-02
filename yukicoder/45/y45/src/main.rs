use std::io::*;

fn read_line()->String{
    let mut buffer = String::new();

    let x = std::io::stdin().read_line(&mut buffer).ok();

    buffer
}


fn main() {

    let num = String::from(read_line().trim());

    let dd = read_line();

    let dishes:Vec<_> = dd.trim().split(" ").collect();

    let mut max_value:[i32;2] = [0, 0];
    
    //println!("{}", num);

    for d in dishes {
        let mut max_v:[i32;2] = [0, 0];
        //max_value[0]は1つ前の皿を取らなかった場合の最大値
        //max_value[1]は1つ前の皿を取った場合の最大値
        //d-1
        max_v[0] = std::cmp::max(max_value[0], max_value[1]);
        //d-2と現在の皿を選んだ場合
        max_v[1] = max_value[0] + d.parse::<i32>().unwrap();

        max_value[0] = max_v[0];
        max_value[1] = max_v[1];
        // 1, 2, 3, 4, 5
    }
    let result = std::cmp::max(max_value[0], max_value[1]);

    println!("{}", result);

}
