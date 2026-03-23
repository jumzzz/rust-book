# On Message Passing

## Interesting Observation

We have this code
```rust
// mpsc = Multiple Producer, Single Consumer
use std::sync::mpsc;
use std::thread;

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
```

Which outputs
```
sending val to recv0 = Message Thread #1
recv0 has now been received.
sending val to recv1 = Message Thread #2
recv1 has now been received.
sending val to recv2 = Message Thread #3
recv2 has now been received.
message from tx0 = Message Thread #1 
message from tx1 = Message Thread #2 
message from tx2 = Message Thread #3 
```

### Interpretation
- `rx0.recv().unwrap()` does the thread blocking to you. So you don't have to manually invoke `handle.join.unwrap()`
- Further coding experiments shows that when you send a message to transmission, it's treated as some `queue`


## Message Passing Iteratively

We have this code

```rust
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
```

This produces an output like this
```
sending val = message #1 from producer thread.
rx = message #1 received from main thread.
sending val = message #2 from producer thread.
rx = message #2 received from main thread.
sending val = message #3 from producer thread.
rx = message #3 received from main thread.
sending val = message #4 from producer thread.
rx = message #4 received from main thread.
sending val = message #5 from producer thread.
rx = message #5 received from main thread.
```
- This looks like that tx/rx are being synchronized but that's not really the case.
- Now let's comment out `thread::sleep(Duration::from_secs(1))`

```rust
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
            // thread::sleep(Duration::from_secs(1));
        };
    });

    for received in rx {
        println!("rx = {} received from main thread.", received);
    }
}
```
- We'll have output like this
```
sending val = message #1 from producer thread.
sending val = message #2 from producer thread.
sending val = message #3 from producer thread.
sending val = message #4 from producer thread.
sending val = message #5 from producer thread.
rx = message #1 received from main thread.
rx = message #2 received from main thread.
rx = message #3 received from main thread.
rx = message #4 received from main thread.
rx = message #5 received from main thread.
```
- This clearly shows that `tx` just enqueues the message to the channel and `rx` just dequeues it
- The thread sleep gives an illusion of synchronization so we have to be careful in designing message passing between threads.


## Using Multiple Producers
- If you want to use the same `tx`, you can clone `tx` multiple times
- In this way, you won't violate Rust's ownership

```rust
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
```
- This produces an output like this

```
sending msg0 = Message #0
sending msg1 = Message #1
rx = Message #0 received from main thread.
rx = Message #1 received from main thread.
sending msg2 = Message #2
rx = Message #2 received from main thread.
```

### Note:
- The main `tx0` should be unwrapped as well with `tx0.send(msg).unwrap()`, or else `rx` will keep waiting.
- It turns out that `rx` will only be dropped if and only if `tx` goes out of scope.