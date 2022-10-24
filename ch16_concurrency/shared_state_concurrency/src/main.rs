use std::sync::{Arc,Mutex};
use std::thread;

fn basic_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn mutex_as_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;    
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result = {:?}", *counter.lock().unwrap());
}

fn mutex_as_vect() {
    let message_queue = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..10 {
        let message_queue_clone = Arc::clone(&message_queue);
        let handle = thread::spawn(move || {
            let mut message_queue_ptr  = message_queue_clone.lock().unwrap();
            message_queue_ptr.push(format!("current_message #{}", i));   
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result = {:?}", *message_queue.lock().unwrap());
}


fn main() {
    mutex_as_vect();
}

