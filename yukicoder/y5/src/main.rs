
fn read_line()->String{
    let mut buf = String::new();

    let x = std::io::stdin().read_line(&mut buf);

    buf
}


fn main() {

    let max:i32 = read_line().trim().parse::<i32>().unwrap();

    let n:i32 = read_line().trim().parse::<i32>().unwrap();

    let blocks = String::from(read_line().trim());

    let v = blocks.split(" ");

    let mut num_vect: Vec<i32> = Vec::new();

    for i in v {
        num_vect.push(i.parse::<i32>().unwrap());
    }

    num_vect.sort();

    let mut count = 0;
    let mut sum = 0;

    for i in num_vect{
        sum += i;

        if sum <= max {
            count += 1;
        }
    }
    println!("{}", count);

}
