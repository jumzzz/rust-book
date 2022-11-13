extern "C" {
    fn abs(input: i32) -> i32;
    fn fabs(input: f64) -> f64; 
}


fn main() {
    unsafe {
        println!("C abs({}) = {}", -3, abs(-3));
        println!("C fabs({}) = {}", -2.5, fabs(-2.5));
    }

}
