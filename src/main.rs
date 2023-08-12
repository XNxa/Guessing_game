use rand::{thread_rng, Rng};
use std::{io, str::FromStr, fmt::Display};

fn ask_a_number<T : FromStr>(msg: &str) -> T 
    where <T as FromStr>::Err: Display 
{
    loop {
        println!("{}", msg);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("A message should be written");

        match input.trim().parse::<T>() {
            Ok(ans) => {return ans;},
            Err(err) => {
                eprintln!("Error : {}", err);
                continue;
            },
        }
    }
}

fn main() {

    // Number generator
    let mut rng = thread_rng();
    
    
    // Asking for min and max
    let mut min:i32;
    let mut max:i32;
    loop {
        min = ask_a_number("Please enter the min value : ");
        max = ask_a_number("Please enter the max value : ");
        
        if min < max {break;} else {
            println!("Max value must be greater than min value. Please try again.")
        }
    }

    // Choose a number randomly
    let n = rng.gen_range(min..=max);

    let mut attempts = 0;
    
    // do while (not the right number)
    loop {
        let choosen:i32 = ask_a_number(&format!("Choose a number between {min} and {max} : "));
        attempts += 1;

        // Compare and tell the user
        if choosen < n {
            println!("too small...");
        } else if choosen > n {
            println!("TOO BIG !")
        } else {
            println!("Congrats ! It was : {n} !");
            println!("It took you {attempts} attempt(s)...");
            break;
        }
    }

}
