use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_input() -> u32 {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                get_input()
            }
        },
        Err(_) => {
            println!("Please enter a valid number");
            get_input()
        }
    }
}

fn main() {
    let target = rand::thread_rng().gen_range(1..=101);
    loop {
        let input = get_input();
        match input.cmp(&target) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
