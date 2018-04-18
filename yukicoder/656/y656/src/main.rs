use std::io::*;
use std::str::*;

fn read_line()->Result<String>{
    let mut buf = String::new();

    let x = stdin().read_line(&mut buf)?;

    return Ok((buf));

}

fn main() {
    let holes = read_line().unwrap().trim();
    let mut sum:i32 = 0;
    for i in holes.chars(){
        let x = i.to_string();
        sum += i32::from_str(x).unwrap();
    }
    print!("{}", sum);
}
