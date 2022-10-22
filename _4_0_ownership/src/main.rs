/**
 * Ownership is used to manage data in the heap. simple data items are usually saved in the stack
 * and can be accessed easily. Also string literals are saved in the stack as their size is known at
 * compile time.
 * 
 * Ownership rules are:
 *      1. Every value has an owner
 *      2. Only one owner at a time
 *      3. When owner goes out of scope, value is dropped
 * 
 * Complex data structures such String are saved at heap since memory size is not known at compile time
 * and one has to request allocation of memory. When out of scope the memory has to be returned to 
 * the allocator.
 * 
 * In stack it is easy to copy element values. eg. 
 *      let x = 5;
 *      let y = x;
 * 
 * Both x and y have same value but dont point to the same memory location. The value of x was copied to y. And thus
 * if we change value of x to 10, y still remains at 5.
 */
fn main() {
    stack_demo();

    heap_move();

    heap_clone();

    test_onweship();
}

// demonstration of move for stack types
fn stack_demo() {

    println!("\n>>>>>>>> stack demo >>>>>>>>>");
    let mut x = 5;
    let y = x;

    println!("Value of x: {x}");
    println!("Value of y: {y}");

    x = 10;


    println!("\nValue of x: {x}");
    println!("Value of y: {y}");
}


// move in heap
fn heap_move() {

    println!("\n>>>>>>>> heap move >>>>>>>>>");
    // hello is created in the heap and s1 added to the stack and cotains pointer, length and capacity
    let s1 = String::from("hello"); 

    // s1 data is copied to s2 and moved, therefore making s1 invalid and not accessible. This ensures that
    // only s2 will drop when out of scope thus not cleaning memory twice which can lead to security vulnerabilities
    // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    let s2 = s1;

    // cannot access s1 as it is invalid - compile time error since you are trying to borrow and moved value
    // println!("s1 value: {s1}"); // value borrowed here after move

    println!("s2 value: {s2}"); // s2 is valid - only one owner at a time
}

// this is similar to deep copy and copies that values rather metadata. Rust never deeply copies elements thus
// fast at runtime unless explicitly state so.
fn heap_clone() {
    println!("\n>>>>>>>> heap clone >>>>>>>>>");
    let mut s1 = String::from("hello");
    let s2 = s1.clone(); // contains a deep copy of s1

    let is_equal = s1 == s2; // true
    println!("is {s1} equal to {s2}: {is_equal}");

    s1 = String::from("world");

    let is_equal = s1 == s2; // false
    // only s1 value is changed since they reference to different values in heap.
    println!("After mutation: is {s1} equal to {s2}: {is_equal}"); 
}

fn test_onweship() {
    println!("\n>>>>>>>> Test Ownership >>>>>>>>>");
    let s = String::from("hello"); // s comes into scope
    take_ownership(s); // s moves

    // from here s is invalid and results in compile error
    // println!("Value of s: {s}"); // erro value borrowed here after move

    let x = 5; // x comes into scope
    make_copy(x); // a copy x is created and passed to make copy function

    // using x here is still valid unlike s above
    println!("Origin value of x is {x}");
}

fn take_ownership(s: String) { // string s comes into scope
    // s is valid here
    println!("owned string here : {s}");
} // s goes out of scope and is dropped, drop method is called and memory freed

fn make_copy(x: i32) { // integer x comes into scope
    // use x
    println!("Value of x: {x}")
} // x goes out of scope -- nothing special happens