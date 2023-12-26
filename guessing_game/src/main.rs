use std::io;
use rand::Rng; // Rng trait defines methods that random number generator implement, 
               // so this trait must be in score to use those methods
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // gen_range is defined in the trait that was brought into scope with use rand::Rng;
    // gen_range takes a range expression and generates a random number within that range
    // the range expression is `start..=end`, meaning inclusive end with stepsize 1
    // by default. It is defined for Idx traits. Traits are like Haskell's typeclasses
    
    // println!("The secret number is: {secret_number}");
    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); 
        // guess has a type! It is inferred by the compiler. Rust is strongly typed after all.
    
        io::stdin()
            .read_line(&mut guess) // returns Result value, which is an enum with variants Ok and Err
            .expect("Failed to read line"); // if Err, "panics"
        
        // A rather primitive way of parsing user input:
        
        // let guess : u32 = guess.trim().parse().expect("Please type a number!"); 
        
        // trim() eliminates leading and trailing whitespaces
        // parse() will try to parse the expression. Rust cannot infer to what it
        // should parse guess, so that is why typing is strictly necessary here.
    
        // again, expect() "panics" on variant Err. On Ok, it will unwrap the u32 wrapped in
        // Ok and pass it to guess.
        // "panicing" means terminating the program: especially for user interaction, this is
        // not an elegant way of handling misformatted input: the user should get a second try.
    
        // Also note the typing syntax: it is much like Python's typehints.
        
        // A more elegant way of handling errors:
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue, // go back to top of loop
        };
        // Here we see again a correspondence with Haskell's Either type
        // Second, notice that `match` can have the semantics of a statement (like below)
        // or an expression (like here above).
    
        println!("You guessed: {guess}");


        // match is the rust equivalent of a switch statement. 
        // it matches cases of an enumerable type.
        // Ordering is an enum with variants Less, Greater, Equal,
        // much like Haskell's Ordering = LT | EQ | GT
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // unused variables are warned for unless 
    // you name them with a preceding underscore
    let _apples = 5; // immutable
    let mut _bananas = 5; // mutable    
    _bananas = 6; 
}
