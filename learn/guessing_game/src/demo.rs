use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secret_number}");

    loop {
        println!("please input your guess");


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}oep you get it in rihgt way so 
