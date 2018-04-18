use std::io::*;

fn read_line()->Result<String>{
    let mut buffer = String::new();

    let x = stdin().read_line(&mut buffer)?;

    Ok((buffer))

}

fn main() {
    let num = (String::from(read_line().unwrap().trim())).parse::<u32>().unwrap();

    let mut a = 1;
    let mut counter = 0;

    while( a < num ){
        counter += 1;
        a *= 2;
    }

    println!("{}", counter);
}
