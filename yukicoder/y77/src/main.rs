fn get_line()->String{
    let mut buf = String::new();
    let x = std::io::stdin().read_line(& mut buf);
    buf
}


fn main() {
    let s = get_line();
    let n = s.trim().parse::<i32>().unwrap();

    let s = get_line();
    let x: Vec<&str> = s.trim().split(" ").collect();

    let mut lengas: Vec<i32> = Vec::new();

    for (i, item) in x.iter().enumerate(){
        lengas.push(x[i].parse::<i32>().unwrap());
    }

    // ���̈ʒu��ݒ�
    // �s���~�b�h�̑傫����ݒ�
    
    for i in 1..n{ // �T���͈�
        

    }


}
