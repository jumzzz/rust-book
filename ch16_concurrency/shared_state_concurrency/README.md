# Shared State Concurrency

## Key Insights
- `Mutex<T>` can't be moved since it doesn't share copy trait
- `Rc<T>` is not thread-safe so the compiler prevents you from using it in multiple threads
- Use `Arc<T>`

## Using Mutexes as a Counter

```rust
use std::sync::{Arc,Mutex};
use std::thread;

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
```
- For the first line, in `counter = Arc::new(Mutex::new(0))`, the value `0` was wrapped inside a mutex `Mutex<T>`, this enable your code to access shared memory without corrupting it.
- In other words, `Mutex<T>` prevents race conditions by locking it.
- Next, we wrap `Mutex<T>` with `Arc<T>`. This is similar to `Rc<T>` but has atomic properties. This allows the same functionality of `Rc<T>` while being thread safe.
- Before passing it inside the `thread::spawn` closure, we first clone the reference with `Arc::clone`. This is similar again with `Rc::clone` which just increases the increment of reference count.
- Now, we can pass the `counter_clone` inside the `handle`'s closure, in that way we won't violate any ownership rules.
- To modify the value of `counter_clone`, you must acquire the lock by doing `counter_clone.lock().unwrap()`, then dereference it by `*num += 1` to make modifications


## Using Mutexes on Heap based data structures like vec!

```rust

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

```
- `mutex_as_vect()` is almost the same with `mutex_as_counter()` with one minor difference.
- Notice that you don't need to derefernce message_queue_ptr since `vect` itself is already a pointer.


## Additional Side Note
- Implementing `Mutex<T>` obviously looks more complex than its Message Passing counterpart.
- One disadvantage is that `Mutex<T>` runs on the risk of `deadlocks`.
- It also has a similar risk of causing Memory Leaks via reference cycles. Use `Mutex<T>` very carefully.
- The API documentation `MutexGuard` offers useful information.