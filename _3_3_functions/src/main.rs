/**
 * Rust is an expression base language. The expression is a statement that returns or evaluates to a value.
 * Statements dont return values
 * 
 * since Rust is expression language below is not valid but is valid in java or C
 * let x = y = 10;
 * 
 * 10 return value as it is an expression but y does not as it being defined...java or c when initializing
 * they return binded value
 */

fn main() {
    let (_x, _) = (10, 32); // this is an expression as let does not return

    plus_one(five())
}

fn five() -> i32 {
    5 // this is an expression as it does not have ;
}

fn plus_one(x: i32) {
    let y = x + 1;
    println!("{x} + 1 = {y}");
}
