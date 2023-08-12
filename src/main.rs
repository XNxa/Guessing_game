use rand::{thread_rng, Rng};
use std::io;

const MIN: i32 = 1;
const MAX: i32 = 1000;

fn main() {

    // Number generator
    let mut rng = thread_rng();
    
    // Choose a number randomly
    let n = rng.gen_range(MIN..=MAX);
    
    // do while not the right number
    loop {
        // Ask user a number
        println!("Choose a number between {MIN} and {MAX} : ");
        
        // IO
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).expect("A message should be writen");

        let choosen = match input.trim().parse::<i32>() {
            Ok(number) => number,
            Err(err) => {
                eprintln!("You should write a number : {err}");
                continue;
            },
        };

        // Compare and tell the user
        if choosen < n {
            println!("too small...");
        } else if choosen > n {
            println!("TOO BIG !")
        } else {
            println!("Congrats ! It was : {n} !");
            break;
        }
    }

}
