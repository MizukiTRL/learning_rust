use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=10);

    loop {

        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num){
            Ordering::Less => println!("smaller"),
            Ordering::Equal => {
                println!("exactly!!!!");
                println!("the secret number is: {secret_num}");
                break;
            }
            Ordering::Greater => println!("too big :("),
        }

    }

    let x =5;
    let y = 8;

    println!("perrero {x} tiene {} gatos", y + 1);
}

