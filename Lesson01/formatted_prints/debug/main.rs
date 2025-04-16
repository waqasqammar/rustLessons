
// The Unprintable cannot be printed with either fmt::Display or fmt::Debug
#[allow(dead_code)]
struct Unprintable(i32);

// The derive attribute automatically create the implemention for the fmt::Debug for Printable
#[derive(Debug)]
struct Structure(i32);
fn main () {
    
    println!("{:?} month in a year", 12);
    println!("{:?} is yonger than {:?}", "Alice", "Bob");

    println!("{:?} is a printable struct", Structure(32));

    //pretty printable
    println!("{:#?} is a struct formatted with {:?}", Structure(9), "Pretty Printing");
}