#[allow(dead_code)]
fn crash_and_burn() {
    panic!("Crash and Burn!");
}

#[allow(dead_code)]
fn panic_backtrace() {
    let mut v = vec![1, 2, 3];
    v[99] = 10;
}

fn main() {
    panic_backtrace();
}
