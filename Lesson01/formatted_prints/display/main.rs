use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

// Implement fmt::Display for Complex
impl fmt::Display for Complex {
    // This trait requires fmt with this exact signature 
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} + {:.2}i", self.real, self.imag)
    }
}
fn main () {

    println!("Complex Debug: {:?}", Complex{real: 32., imag: 54.});
    println!("Complex Display: { }", Complex{real: 3.2594, imag: 7.5343});
}