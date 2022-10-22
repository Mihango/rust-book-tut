fn main() {
    tuple_ownership();

    reference_sample();
}

// due to ownership issues whenever you move a value to a funtion, to use it
// you can return it again, and if need to return another values after process, you can use a
// tuple
fn tuple_ownership() {
    println!("\n>>>>>>>>>>> tuple ownership >>>>>>>>>>>");
    let s = String::from("Hello"); // s into scope

    let result = caluculate_length(s); // s is moved here
    // s is invalid here and cannot be used but can be accessed from the tuple
    println!("Length of {} is: {}", result.0, result.1);
}

fn caluculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// using tuple to return values whenever you want to reuse them is not scalable and may decrease readability
// thus Rust provides a solution for this using references as below
fn reference_sample() {
    println!("\n>>>>>>>>>>> reference >>>>>>>>>>>");
    let s = String::from("Hello"); // s into scope

    // the action of creating a reference is call borowwing in rust i.e have borrowed s below
    let len = caluculate_length_ref(&s); // ownership is not move and use a reference/pointer which returns usize
    // s is valid here
    println!("Length of {s} is: {len}");

}

fn caluculate_length_ref(s: &String) -> usize {
    s.len()
} // s goes out of scope but since it does not own the value referenced, it is not dropped

// Rules of reference
//      1. You can only have one mutable reference or several immutable reference -- to have them their lifetimes must complete 
//          the mutable
//      2. Reference must always be valid -- you can not reference a value that has already been dropped

// &str string slice eg string literal can be used to reference and a string or an array and ensure that value is not 
// dropped when it is being reference -- eg when getting first word separated by space in a string, return the index of it, and
// then clearing the string my bring problem if we try to use it when we have cleared the string
// to avoid this scenario - use string slices which will ensure that clearing os s does not happend as it does not allow mutable
// borrow when you have an immutable reference