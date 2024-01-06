use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the number of rows:");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let rows: usize = input.trim().parse().expect("Invalid inpt");

    for i in (1..=rows).rev() {
        for _ in 0..i {
            print!("*");
        }
        println!("");
    }
}