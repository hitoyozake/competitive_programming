


// 標準入力一行を読み取り，指定の型に変換する関数
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_splited<T: std::str::FromStr>() -> Vec<T>{

    let mut s = String::new();

    std::io::stdin().read_line(&mut s).ok();

    s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()

}




fn main(){

    let s: String = read();
    let mut v = Vec::new();

    for c in s.chars() {
        match c {
        '0' => {
            v.push(c);
        },
        '1' => {
            v.push(c);
        }
        'B' => {
            if v.len() > 0{
                v.pop();
            }
        }
        _ => println!("ERROR!")

        }

    }


    for i in v {
        print!("{}", i);
    }
    print!("\n");
    //let f = format!("{}", s);


    //println!("{}", f);
}

