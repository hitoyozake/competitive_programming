
fn get_line()->String{
    let mut x  = String::new();
    let r = std::io::stdin().read_line(&mut x);

    x
}


fn main() {

    let nlines = String::from(get_line().trim());

    let nums: Vec<&str> = nlines.split(' ').collect();

    
    let y: u32 = nums[0].parse::<u32>().unwrap(); 
    let niku: u32 = nums[1].parse::<u32>().unwrap();
    let times: u32 = nums[2].parse::<u32>().unwrap();
    let maxsum:u32 = nums[3].parse::<u32>().unwrap();

    let answer = std::cmp::min(niku/times, y); //肉
    //println!("{}", answer);
    let ans2 = std::cmp::min(maxsum/(1+times), answer); // 合計数の最小
    //println!("{}", ans2);
    let ans3 = std::cmp::min(ans2, y); // 野菜


    println!("{}", ans3);
}
