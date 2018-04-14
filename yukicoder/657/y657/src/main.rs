
use std::io::*;
use std::str::*;
use std::io::{self, Read};

fn get_line()->Result<String>{
    let mut buffer  = String::new();

    let x = io::stdin().read_line(&mut buffer)?;

    Ok((buffer))

}

fn tetra(n:u32)->u32{
    match n 
    {
        0 => 0,
        1 => 0,
        2 => 0,
        3 => 0,
        4 => 1,
        _ => { tetra(n-1) + tetra(n-2) + tetra(n-3) + tetra(n-4) },
    }
}


fn main() {
    let n = i32::from_str(get_line().unwrap().trim()).unwrap();

    let mut v :Vec<u32>= Vec::new();

    for i in 0..n {
        let m = u32::from_str(get_line().unwrap().trim()).unwrap();
        let t = tetra(m)%17;
        v.push(t);
    }

    for i in v {
        println!("{}", i);
    }

}
