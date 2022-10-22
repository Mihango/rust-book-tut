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
    let len = caluculate_length_ref(&s); // ownership is not move and use a reference/pointer which returns usize
    // s is valid here
    println!("Length of {s} is: {len}");

}

fn caluculate_length_ref(s: &String) -> usize {
    s.len()
}