## Move in Rust


## Copying Values of stack allocated variables
If a variable memory allocation can be predetermined at compile-time (stored at Stack)

```rust
let stack_var0 = 420;

// This is legal

let stack_var1 = stack_var0;
let stack_var2 = stack_var0;
let stack_var3 = stack_var0;
let stack_var4 = stack_var0;
let stack_var5 = stack_var0;
let stack_var6 = stack_var0;

// No special actions needed here since this will copy everything (deep copy)
// And each of these values will have its own memory address

println!("stack_var0 = {:p}", &stack_var0);     // Has its own address
println!("stack_var1 = {:p}", &stack_var1);     // Different address to stack_var0
println!("stack_var2 = {:p}", &stack_var2);     // Different address to stack_var1
println!("stack_var3 = {:p}", &stack_var3);     // Different address to stack_var2
println!("stack_var4 = {:p}", &stack_var4);     // Different address to stack_var3
println!("stack_var5 = {:p}", &stack_var5);     // Different address to stack_var4
println!("stack_var6 = {:p}", &stack_var6);     // Different address to stack_var5

```

## Difference of usage between Stack and Heap Memory

- If the memory block size of a variable/object can be predetermined at compile-time, it should be allocated on stack
- If the memory block size of a variable/object cannot be determined at compile-time, it should be allocated on the heap

## Copying/Moving Values of heap allocated variables

If a variable is allocated at the heap, then this would happen

```rust
let heap_var0 = String::from("This is allocated on Heap");

// This will compile but heap_var0 will be out of scope
let heap_var1 = heap_var0;

// After assigning heap_var0, you can no longer do this, thus this won't compile
let heap_var2 = heap_var0;      // This will not compile

```

## Forcing Heap allocated variables/object to copiable (Deep Copy)

You can allow heap allocated variables to be copiable with the annotation `#[derive(Copy, Clone)]`, here are some example

```rust
// Allowing copy, clone
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

fn copy_values() {

    let mut p1 = Point{ x: -1, y: 2 };
    let p2 = p1;
    
    println!("Invoking copy_values()");
    println!("p1: {:p}", &p1);
    println!("p2: {:p}", &p2);

}
```
## Comments
- Not to sound I'm biased with Rust but this is quite a logical rule that allows you to strictly enforce ownership at compile time
- If these ownership rules are violated, the compiler **WILL** detect it.


