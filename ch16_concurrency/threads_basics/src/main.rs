use std::thread;
use std::time::Duration;


#[allow(dead_code)]
fn basic_threading() {

    thread::spawn(|| {
        for i in 1..100 {
            let thread_id = thread::current().id();
            println!("[thread_id = {:?}] currently running index = {} at 1st thread", thread_id, i);
        }
    });

    thread::spawn(|| {
        for i in 1..100 {
            let thread_id = thread::current().id();
            println!("[thread_id = {:?}] currently running index = {} at 2nd thread", thread_id, i);
        }
    });

    for i in 1..100 {
        let thread_id = thread::current().id();
        println!("[thread_id = {:?}] currently running index = {} at main thread", thread_id, i);
    }
}


/// handle.join().unwrap() blocks other threads from executing until the thread
/// that's associated with the handle finishes
#[allow(dead_code)]
fn block_threads() {
    let handle_01 = thread::spawn(|| {
        for i in 1..5 {
            let thread_id = thread::current().id();
            println!("[thread_id = {:?}] currently running index = {} at 1st thread", thread_id, i);
        }
    });

    handle_01.join().unwrap();
    
    let handle_02 = thread::spawn(|| {
        for i in 1..5 {
            let thread_id = thread::current().id();
            println!("[thread_id = {:?}] currently running index = {} at 2nd thread", thread_id, i);
        }
    });

    handle_02.join().unwrap();

    let handle_03 = thread::spawn(|| {
        for i in 1..5 {
            let thread_id = thread::current().id();
            println!("[thread_id = {:?}] currently running index = {} at 3rd thread", thread_id, i);
        }
    });

    handle_03.join().unwrap();
}


#[allow(dead_code)]
fn threads_move_data() {
    let v = vec![1, 2, 3];    

    let handle = thread::spawn(move || {
        println!("Here's a vector = {:?}", v);
    });

    // Since thread handle now owns v
    // You cannot do things right now on v
    // The lifetime of v ended at the scope of 
    // handle

    handle.join().unwrap();
}

fn main() {
    // basic_threading();
    // block_threads();
    threads_move_data();
}
