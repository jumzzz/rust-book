fn growing_vec() {
    let size = 50 * 1024 * 1024;
    let mut x: Vec<usize> = Vec::new();

    for i in 0..size {
        x.push(i);
    }
    println!("{:?}", x.iter().sum::<usize>());
}

fn growing_vec_v2() {
    let size = 50 * 512 * 512;
    let mut x: Vec<usize> = Vec::new();

    for i in 0..size {
        x.push(i);
    }
    println!("{:?}", x.iter().sum::<usize>());
}

fn growing_vec_v3() {
    let size = 50 * 256 * 256;
    let mut x: Vec<usize> = Vec::new();

    for i in 0..size {
        x.push(i);
    }
    println!("{:?}", x.iter().sum::<usize>());
}




fn main() {
    growing_vec();
    growing_vec_v2();
    growing_vec_v3();

}
