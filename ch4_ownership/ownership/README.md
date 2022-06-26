### Ownership Rules in Rust
- Each value in Rust has a variable that's called its owner
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped

In order to support a mutable, growable piece of text, we need to allocate an
amount of memory on the heap, unknown at compile time, to hold the contents.

The memory must be requested from the memory allocator at runtime
We need a way of returning this memory to the allocator when we're done with out string.

- In Language with GC, the memory keeps being tracked.
- In Language like C/C++ the memory must be manually allocated

Rust takes a different approach, once it goes out of scope, Rust discards this automatically.

### Question Question Question
- Does that mean that when I try to get a dangling pointer it won't be available to me?
    Answer: no
- How about when you return a value or a memory address from a function? Does the ownership transferred?
    Answer: no. i don't know. but certainly not within my assumptions.

### Question again:
-   Does Rust generate a code that invokes malloc/calloc at the end of the scope?
    - Let's try strace target/release/ownership
    - Answer: I only see single `munmap()`

-   There's consistently only two munmap during the strace. This is really weird.
    This is weird because it tells you that the value would be dropped when it is out of scope.
    I probably misunderstanding something. -- If this is the case, the how does Rust does it??

    - By the looks of it, rust really produces a large binary file. No wonder it's assembler is 
      hard to understand.

    - For some reason, there are some magic (with dynamic linking) that facilitates allocation and 
      deallocation of memory in Rust. Anyhow, this is what the Rust binary does when you perform
      ltrace +x target/release/ownership


```shell
ltrace -x "malloc+realloc+free" target/release/ownership

malloc@libc.so.6(472)                                                                            = 0x5609fb5012a0
malloc@libc.so.6(120)                                                                            = 0x5609fb501480
malloc@libc.so.6(1024)                                                                           = 0x5609fb501500
realloc@libc.so.6(0x5609fb501480, 240)                                                           = 0x5609fb501910
free@libc.so.6(0x5609fb501910)                                                                   = <void>
free@libc.so.6(0x5609fb501500)                                                                   = <void>
free@libc.so.6(0x5609fb5012a0)                                                                   = <void>
realloc@libc.so.6(0, 32 <unfinished ...>
malloc@libc.so.6(32)                                                                             = 0x5609fb501a10
<... realloc resumed> )                                                                          = 0x5609fb501a10
free@libc.so.6(0x5609fb501a10)                                                                   = <void>
malloc@libc.so.6(5)                                                                              = 0x5609fb501a40
malloc@libc.so.6(48)                                                                             = 0x5609fb501a60
malloc@libc.so.6(32)                                                                             = 0x5609fb501a10
realloc@libc.so.6(0x5609fb501a10, 64)                                                            = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 128)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 256)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 512)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 1024)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 2048)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 4096)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 8192)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 16384)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 32768)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 65536)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 131072)                                                        = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 262144)                                                        = 0x7f0348d8b010
realloc@libc.so.6(0x7f0348d8b010, 524288)                                                        = 0x7f0348d0a010
realloc@libc.so.6(0x7f0348d0a010, 1048576)                                                       = 0x7f0348c09010
realloc@libc.so.6(0x7f0348c09010, 2097152)                                                       = 0x7f0348a08010
realloc@libc.so.6(0x7f0348a08010, 4194304)                                                       = 0x7f0348607010
realloc@libc.so.6(0x7f0348607010, 8388608)                                                       = 0x7f0347e06010
realloc@libc.so.6(0x7f0347e06010, 16777216)                                                      = 0x7f0346e05010
realloc@libc.so.6(0x7f0346e05010, 33554432)                                                      = 0x7f0344e04010
realloc@libc.so.6(0x7f0344e04010, 67108864)                                                      = 0x7f0340e03010
realloc@libc.so.6(0x7f0340e03010, 134217728)                                                     = 0x7f0338e02010
realloc@libc.so.6(0x7f0338e02010, 268435456)                                                     = 0x7f0328e01010
realloc@libc.so.6(0x7f0328e01010, 536870912)                                                     = 0x7f0308e00010
malloc@libc.so.6(1024)                                                                           = 0x5609fb501500
1374389508505600
free@libc.so.6(0x7f0308e00010)                                                                   = <void>
malloc@libc.so.6(32)                                                                             = 0x5609fb501a10
realloc@libc.so.6(0x5609fb501a10, 64)                                                            = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 128)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 256)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 512)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 1024)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 2048)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 4096)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 8192)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 16384)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 32768)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 65536)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 131072)                                                        = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 262144)                                                        = 0x7f0348d8b010
realloc@libc.so.6(0x7f0348d8b010, 524288)                                                        = 0x7f0348d0a010
realloc@libc.so.6(0x7f0348d0a010, 1048576)                                                       = 0x7f0348c09010
realloc@libc.so.6(0x7f0348c09010, 2097152)                                                       = 0x7f0348a08010
realloc@libc.so.6(0x7f0348a08010, 4194304)                                                       = 0x7f0348607010
realloc@libc.so.6(0x7f0348607010, 8388608)                                                       = 0x7f0347e06010
realloc@libc.so.6(0x7f0347e06010, 16777216)                                                      = 0x7f0346e05010
realloc@libc.so.6(0x7f0346e05010, 33554432)                                                      = 0x7f0344e04010
realloc@libc.so.6(0x7f0344e04010, 67108864)                                                      = 0x7f0340e03010
realloc@libc.so.6(0x7f0340e03010, 134217728)                                                     = 0x7f0338e02010
85899339366400
free@libc.so.6(0x7f0338e02010)                                                                   = <void>
malloc@libc.so.6(32)                                                                             = 0x5609fb501a10
realloc@libc.so.6(0x5609fb501a10, 64)                                                            = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 128)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 256)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 512)                                                           = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 1024)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 2048)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 4096)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 8192)                                                          = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 16384)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 32768)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 65536)                                                         = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 131072)                                                        = 0x5609fb501ad0
realloc@libc.so.6(0x5609fb501ad0, 262144)                                                        = 0x7f0348d8b010
realloc@libc.so.6(0x7f0348d8b010, 524288)                                                        = 0x7f0348d0a010
realloc@libc.so.6(0x7f0348d0a010, 1048576)                                                       = 0x7f0348c09010
realloc@libc.so.6(0x7f0348c09010, 2097152)                                                       = 0x7f0348a08010
realloc@libc.so.6(0x7f0348a08010, 4194304)                                                       = 0x7f0348607010
realloc@libc.so.6(0x7f0348607010, 8388608)                                                       = 0x7f0347e06010
realloc@libc.so.6(0x7f0347e06010, 16777216)                                                      = 0x7f0346e05010
realloc@libc.so.6(0x7f0346e05010, 33554432)                                                      = 0x7f0344e04010
5368707481600
free@libc.so.6(0x7f0344e04010)                                                                   = <void>
free@libc.so.6(0x5609fb501500)                                                                   = <void>
free@libc.so.6(0x5609fb501a40)                                                                   = <void>
free@libc.so.6(0x5609fb501a60)                                                                   = <void>
free@libc.so.6(0x5609fb501aa0)                                                                   = <void>
+++ exited (status 0) +++

```

It shows that it immediately invokes free after it ends at the specific scope. 

It actually confirms my intution. Memory is freed with `free()` after it ends at the scope.
But it's just harder to observe in real time.


-   From the basic rules, it says that there should be a single owner - this implies that the
    ownership is being tracked. IF so, how does Rust tracks its ownership?


### Question:
- When you allocate something in Heap, it returns a memory address.
- If you did an allocation inside a function and returned that address outside, IIRC in C/C++
    you can still access the same memory. Can we verify if this is the same case on Rust?

### Results:

Address output is:

```
    pointer inside experiment_ownership_v1()  = 0x556916632ad0
    pointer outside experiment_ownership_v1() = 0x556916632ad0
```
Hence, memory wasn't moved. It's also flawed because moving memory blocks is costly. That would
be also magic tbh which violates the zero-overhead principle.


### From the Book and Docs
```rust
fn drop(&mut self)
```
- Rust implicitly calls `drop` when the variable goes out of scope
- You cannot explicitly call it. Attempting to do so will result to a compiler error.



### Very Useful Articles I found:

**Investigating Memory Allocations in Rust:** https://ysantos.com/blog/malloc-in-rust
**Rust is not a good C Replacement:** https://drewdevault.com/2019/03/25/Rust-is-not-a-good-C-replacement.html

