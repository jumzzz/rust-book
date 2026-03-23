fn raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {:p}", r1);
        println!("r2 = {:p}", r2);

        // *r1 = 4;    // This is not allowed since it's in const
        println!("*r1 = {}", *r1);

        *r2 = 6;      // This is allowed as long that the actual variable is mutable
        println!("*r2 = {}", *r2);
    }
}

// Needs another unsafe block to be called
unsafe fn dangerous() {}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}


fn main() {
    raw_pointers();

}
