use std::fmt;

// Define a struct named `List`
struct List(Vec<i32>);

// Implement fmt::Display for List
impl fmt::Display for List {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        let vec = &self.0;
        writeln!(f, "List fmt::Display is List: [index: value]")?;
        // start the bracket
        write!(f, "List: [")?;

        for (count , v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }

            // use `?` operator to return on errors
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main () {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

}