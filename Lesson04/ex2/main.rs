
fn main () {
    let s1 = String::from ("Hello");
    let len = calculate_length(&s1);
    println!("String s1: {s1} and length: {len}");

    let mut s2 = String::from ("Hello");
    change (& mut s2);
    
    println!("String s2: {s2}");

    let r1 = & mut s2; // allowed
    // let r1 = & mut s2; // not allowed.
    println!("String r1: {r1}");
    let r2 = & mut s2; // now allowed because r1 is now out of scope
    println!("String r2: {r2}");

    let mut s3 = String::from ("Hello");

    let r3 = &s3; // immutable reference allowed
    let r4 = &s3; // multiple immutable reference alllowed
    // let r5 = & mut s3; // mutable and imutable reference are not allowed in the same scope
    println!("r3: {r3} and r4: {r4}");

     let r5 = & mut s3; // now allowed because `r3` and `r4` are out of scope now.
     println!("String r5: {r5}");
}

fn calculate_length (s: &String) -> usize { // `s` is the reference to the String
    s.len()
}// s goes out of scope here but the value it is pointed to, do not drop because it is not the owner.

fn change(s: &mut String) {
    s.push_str(" ,world");
}