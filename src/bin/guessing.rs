use std::io; // input || output
use std::cmp::Ordering;
use rand::Rng; // rand::Rng is thread_rng trait;using cargo doc --open to see;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // mut variable let foo = 5 foo Immutable
        io::stdin() // read_line => io:Result => [Ok, Err]:enum Types
            .read_line(&mut guess) // use std::io => std::io::stdin   read_line get user input value; & -> using mut guess -> user input => mut guess;
            .expect("Failed to read line"); // Result => Err => expect => output => error; Result => Ok => expect => output => Ok value;
//    let guess: u32 = guess // :u32 assign type -> 100;Rust default -> i32 -> 100.00
//        .trim() // hum.... its easy remove \n
//        .parse() // parse to u32
//        .expect("Please type a number!"); // process parse error
        let guess:u32 = match guess.trim().parse() { // parse => Result Types => [Ok, Err]
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess); // {} placeholder
        match guess.cmp(&secret_number) {
            // Ordering => [Less, Greater, Equal]
            Ordering::Less => println!("Too samill!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
