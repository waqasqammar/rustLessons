# Ownership
* Ownership is a set of rule that governs how a Rust program manages memory.

# Ownership Rules.
1. Each value in `Rust` has a owner.
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
4. Rust calls `drop` function automatically when the scope ends. e.g. `}`. This is similar to `RAII` concept of cpp.
```rs

fn main () {
    {
        let s = String::from("Hello World"); // do the allocation in heap
    } // Rust will call `drop` here to deallocate memory.
    // This is error because  `s` is out of scope here.
    // println!("{}", s); 

    let s1 = String::from("hello");
    let s2 = s1; // This only copies the String data which is on stack. e.g pointer, len, capacity.
    
    // this is error because value of s1 being borrowed after it is moved to s2 in line above.
    // println!("{s1}, world!"); 

    let mut s = String::from("hello");
    s = String::from("ahoy"); // rust will call drop here and free original value's memory immidiately.

    let x = 5;
    let y = x; // rust will call `copy` trait here.

    println!("x = {x}, y = {y}");
}
```
## Ownership and Function
For the concept invloves, check the `ex1` example.

## Reference and Borrowing.
* A reference is like a pointer which can be used to access the data stored at that addres. The data is owned by some other variable. Unlike pointer, a reference is guaranteed to point to a valid value of a particular type for the life time of that reference.
* Creating a reference is called `borrowing`. 
* References are immutable by default.
* References can be made mutable using `mut` keyword but they have one big restriction: If you have a mutable reference to a value, you can have no other references to that value. 
* By enforcing above rule, Rust prevent `race condition` which occurs because of the following reasons:
    * Two or more pointers access the same data at the same time.
    * At least one of the pointer being used to write the data.
    * There is no mechanism to sync access to the data.
* So, we cannot have mutable reference if we have immutable reference in the same scope.
* Rust gurantee that there is no dangling pointer at compile time. If there is a reference to the data, the compiler will ensure that data will not go out of scope before the reference.
* At any given time, there can be either one mutable reference or multiple immutable refernces.
* Reference must always be valid. 

## Slice
* String slice refers to as `&str`.
* Use `(s: &str)` in function argument instead of `(s: &String)` to accept both slice and string references.