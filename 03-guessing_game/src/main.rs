use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let secret_number = rand::rng().random_range(1..=100);
    
    
    loop{
        println!("Guess the number\nPlease, input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read the line");
        
        println!("Your guess: {guess}");

        let guess: u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=> continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small :("),
            Ordering::Greater => println!("Too BIG >:)"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    };
    

}