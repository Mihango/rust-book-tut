/**
 * loops and condition control the flow of code
 * loops: loop, for, while
 * condition: if, else if, else, match
 */
fn main() {
   // if else is an expression thus return a value
   let num = 3;

   // _is_more is bound to a bool value from if expression
   let _is_more = if num > 3 {true} else {false};

   // when looping you can also return a value when breaking
   let mut counter = 2;
   let count = loop {
       counter += 1;

       if counter == 10 {
        break counter * 3;
       }
   };
   println!("value of 2300 % 3 = {count}");

   // for loop
   let arr = [10, 30, 40, 50, 70];
   for element in arr {
    println!("element {element}");
   }

   range_loop();
}

fn range_loop() {
    println!("****** Range ******");

    for num in (1..4).rev() {
        println!("{num}, ")
    }
}
