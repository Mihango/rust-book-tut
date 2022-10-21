
/**
 * Rust has two data types: 
 * - scalar: stores single values; They are 4 types: integer, floating point number, Boolean, character
 * - complex: tuples, arrays: tuples cannot increase in size when declared
 */
fn main() {
    interger_representation();

    complex_types();

    test_binary();
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

fn complex_types() {
    // tuple is a way to group together variety of values into one compound type
    let tuple: (u32, f32, char, bool) = (500, 56.9, 'x', true);

    // an empty tuple is called a unit and is what expression returns by default -- type: ()
    let _unit_tuple: () = (); 

    // you can destructure tuple as follows
    let (_num, _float_num, _ch, _is_bool) = tuple;

    println!("complex tuple >>>> {:?}", tuple);

    // mutable tuple
    let mut tuple = (3, 5);
    tuple.0 = 10;
    println!("mutable tuple >>>> {:?}", tuple);

    // define and initialize arrays with elements
    let arr: [i32; 5] = [3; 5]; // create an array of 5 elements containing 3's
    println!("sample array >> {:?}", arr);

    let _arr: [i32; 5];
    // calling arr below without initialization causes compile error
    // println!("sample array >> {:?}", _arr);
}

fn test_binary() {
    // similar to tuple only that all elements are of the same type and have fixed length
    let arr = [1_u32, 3, 4, 6, 32, 45, 67, 87, 503];
    let start: usize = 0;
    let end: usize = arr.len() - 1;
    let target: u32 = 6;

    let index = binary_search_rec(&arr, target, start, end);
    println!("Recursive search: index of {target} in {:?} is {index}", arr);

    let index = binary_search_iter(&arr, target);
    println!("Iterative search: index of {target} in {:?} is {index}", arr);
}

fn binary_search_iter(arr: &[u32], target: u32) -> i32 {
    let mut start: usize = 0;
    let mut end: usize = arr.len() - 1;

    while start <= end {
        let mid = start + (end - start) / 2;

        if target == arr[mid] {
            return mid as i32;
        } else if arr[mid] < target {
            start = mid + 1;
        } else {
            end = mid -1;
        }
    }

    return -1;
}

fn binary_search_rec(arr: &[u32], target: u32, mut start: usize, mut end: usize) -> i32 {
    if start > end {
        return -1;
    } 
    
    let mid = start + (end - start) / 2;

    if target == arr[mid] {
        return mid as i32;
    } else if arr[mid] < target {
        start = mid + 1;
    } else {
        end = mid -1;
    }

    return binary_search_rec(arr, target, start, end);
}
