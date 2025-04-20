
fn main () {
    let s = String::from("hello"); // `s` comes into scope

    take_ownership(s); // `s` value moved into the function.

    // `s` is not valid anymore

    let x: i32 = 5;
    makes_copy(x);
    println!("orignal: {x}"); // still valid because `x` is i32 and it impelments `copy` traits.

    let s1 = give_ownership(); // `give_ownership` moves its return value to `s1`
    println!("s1: {s1}");

    let s2 = String::from("Hello s2"); // s2 comes into scope

    let s3 = take_and_give_back(s2); // `s2` moves into function and function return value moves into `s3`
    // println("{s2}"); // error because s2 is moved so cannot be borrowed
    println!("s3: {s3}"); // s3 is valid
    
}

fn take_ownership (st: String) { // `st` string comes into scope here
    println!("{st}");
} // here `st` string goes out of scope and `drop` is called.

fn makes_copy (it: i32) { // `it` integer comes into scope.
    println!("copied: {it}");
} // `it` goes out of scope and `drop` is called

fn give_ownership () -> String {
    let some_string = String::from("yours"); // `some_string` comes into scope

    some_string // `some_string` moves out to the calling function
}

fn take_and_give_back(string_a: String) -> String { // `string_a` comes into scop   
    string_a // `string_a` moves out it value to calling function
}