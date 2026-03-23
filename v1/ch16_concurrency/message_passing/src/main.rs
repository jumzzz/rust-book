// mpsc = Multiple Producer, Single Consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn channels_basics() {
    let (tx0, rx0) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Message Thread #1");
        println!("sending val to recv0 = {}", val);
        tx0.send(val).unwrap();
        // This won't compile because ownership was already transferred
        // println!("val has different owner = {}", val);
    });

    thread::spawn(move || {
        let val = String::from("Message Thread #2");
        println!("sending val to recv1 = {}", val);
        tx1.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("Message Thread #3");
        println!("sending val to recv2 = {}", val);
        tx2.send(val).unwrap();
    });

    let recv0 = rx0.recv().unwrap();
    println!("recv0 has now been received.");
    let recv1 = rx1.recv().unwrap();
    println!("recv1 has now been received.");
    let recv2 = rx2.recv().unwrap();
    println!("recv2 has now been received.");
   
    println!("message from tx0 = {} ", recv0);
    println!("message from tx1 = {} ", recv1);
    println!("message from tx2 = {} ", recv2);
    
}

fn channels_iteration() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("message #1"),
            String::from("message #2"),
            String::from("message #3"),
            String::from("message #4"),
            String::from("message #5"),
        ];

        for val in vals {
            println!("sending val = {} from producer thread.", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    for received in rx {
        println!("rx = {} received from main thread.", received);
    }
}

// This is done by doing multiple clones of transmitter
fn multiple_producers() {
    
    let (tx0, rx) = mpsc::channel();

    let tx1 = tx0.clone();
    let tx2 = tx0.clone();

    let msg0 = String::from("Message #0"); 
    let msg1 = String::from("Message #1"); 
    let msg2 = String::from("Message #2"); 

    thread::spawn(move || {
        println!("sending msg0 = {}", msg0);
        tx0.send(msg0).unwrap();
    });

    thread::spawn(move || {
        println!("sending msg1 = {}", msg1);
        tx1.send(msg1).unwrap();
    });

    thread::spawn(move || {
        println!("sending msg2 = {}", msg2);
        tx2.send(msg2).unwrap();
    });

    for received in rx {
        println!("rx = {} received from main thread.", received);
    }
}

fn main() {
    // channels_basics();
    // channels_iteration();
    multiple_producers();
}
