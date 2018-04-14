use std::io::*;
use std::str::*;
use std::io::{self, Read};

///
///
///read_lineは改行が入るので注意が必要
fn getline() -> Result<(String)> {
    let mut buffer = String::new();
    // ? はtry!の糖衣構文
    let x = io::stdin().read_line(&mut buffer)?;
    //println!("{}", x); // xは入力byte数
    Ok((buffer))
}

fn main() {
    let n = getline();
    let mut vec = Vec::new(); 
    match n {
        Ok(buf) => {
            let m :u32 = buf.trim().parse::<u32>().unwrap();
            let mut a = "";  
            for i in 0..m{
                let apples = getline();

                match apples{
                    Ok(b) => {
                      let apple_n = (b.trim().parse::<u32>().unwrap())/3; 
                      let row = b.trim().parse::<u32>().unwrap();
                        if row%8 ==0&& row%10==0{
                          vec.push("ikisugi".to_string());
                        }
                        else if row % 8 == 0 {
                            vec.push("iki".to_string());
                        }
                        else if row%10==0{
                            vec.push("sugi".to_string());
                        }
                        else {
                        
                            vec.push(apple_n.to_string());
                        }
                    }
                    Err(err) =>{},
        
                }
            }
        
        }

        Err(err)=>{},
    }
    for i in vec{
        println!("{}", i);
    }

    /*
    let n = getline();
    match n {
        Ok(buf) => {
            println!("input is {}", buf);
            let sp = buf.split(" ");
            for s in sp{
               println!("{}", s);
            }
        }
        Err(error) => println!("error is {}", error),
    }*/

}
