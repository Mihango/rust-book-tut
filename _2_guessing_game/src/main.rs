use std::io; // use statement is used to bring types not in prelude you want to use in the scope
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   println!("****** Guess a number ******");
   let secret_number = rand::thread_rng().gen_range(1..=100);

   loop {
    println!("Please enter a number: ");

    let mut guess = String::new(); // :: indicates association to a type so ::new() function is associated to type String

    // read
    io::stdin()
        .read_line(&mut guess)
        .expect("\nFailed to read line");

        println!("\nYou guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


