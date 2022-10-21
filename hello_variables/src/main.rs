fn main() {
    println!("\n>>>>> START >>>>>>\n");
    // variable definition and immutability
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // function scope constant
    const FUNCTION_CONSTANT: u32 = 60 * 60 * 2;

    println!("\nFun Const: 2 hrs to secs ==> {FUNCTION_CONSTANT}");
    println!("Global Const: 3 hrs to secs ==> {GLOBAL_CONSTANT}");
    
    check_scope();

    shadowing();

    println!("\n>>>>>END>>>>>>\n");
}

/**
 * fails to compile since FUNCTION_CONSTANT is not defined in this
 * scope even though it is a constanct
 */
fn check_scope() {
    println!("\nGlobal Const: 3 hrs to secs ==> {GLOBAL_CONSTANT}");
}

// global constant --> must decalre data type for all constants
const GLOBAL_CONSTANT: u32 = 60 * 60 * 3;

/**
 * Shadowing of variables -> reusing the name of the variable and
 * using keyword let to bind a different value or even a different data type
 */
fn shadowing() {
    let x = 5; // creats variable x and binds 5
    let x = x + 1; // creates another variable x and stores 6

    {
        let x = x * x; // creates another variable and binds 36
        println!("\nInner scope x value: {x}")
    }
    println!("Outer scope x value: {x}")
}
