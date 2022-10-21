
/**
 * Rust has two data types: 
 * - scalar: stores single values; They are 4 types: integer, floating point number, Boolean, character
 * - complex
 */
fn main() {
    interger_representation();
}

/**
 * intergers are reperesnted as either signed or unsigned. signed are represented using two complement
 * method. signed or unsigned indicated whether an integer can ever be negative or positive. An unsigned
 * interger can never be negative
 * 
 * Usigned integers include: u8, u16, u32, u64, u128, usize
 * Signed intergers include: i8, i16, i32, i64, i128, isize
 * 
 * Integers can be represented in different forms: decimal, hexadecimal, ocatl, binary and byte
 * 
 * Integer overflow occurrs when you try to bind a value more than the size of variable can hold: may lead
 * to unrecoverable panic in debug mode while in release mode there will be integer wrapping eg incase of 
 * u32 256 becomes 0, 257 becomes 1 since it can only store numbers from 0 - 255
 */
fn interger_representation() {
    let _x: u32 = "42".parse().expect("Expected a number");

    let num: u8 = 32_u8.wrapping_add(230);
    println!("Wrapped sum of 32 and 230 = {num}");

    let num: (u8, bool) = 32_u8.overflowing_add(230);
    println!("Wrapped sum of 32 and 230 = {:?}", num);

    if num.1 {
        let num: u16 = 255_u16 + (num.0 as u16);
        println!("Wrapped sum of 32 and 230 = {:?}", num);
    }
}
