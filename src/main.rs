use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guesses.");
    loop{

    
    let mut guess = String::new();


    io::stdin().read_line(&mut guess).expect("Failed to load line!");

   

    
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("Please input a number!");
            continue;
        }
    };

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
             println!("You win!");
             break;
        }
        Ordering::Greater => println!("Too big!"),
    }

    }
    
}
