
fn main () {
    // {} replaces with the argument
    println!("{}", 3);

    // positional argument
    println!("{} is big brother of {}", "Alice", "Jon");

    // right justified
    println!("{:>5}", 32);

    // right justified with expression name
    println!("{number:>5}", number=32);

    // pad 0 from left
    println!("{number:0>5}", number=32);

    // pad 0 from right
    println!("{number:0<5}", number=32);
    
    // pad 0 with argument
    println!("{number:0>width$}",number=32, width=5 );

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because Structure does not implement fmt::Display
    // println!("{} will not print ", Structure(32));
    
    // Rust 1.58+ can directly capture the argument from surronding variables
    let number = 32;
    let width = 5;
    println!("{number:>width$}");
}