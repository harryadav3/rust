use std::io;

fn fib_iterative(n: i32) -> i32 {
    let mut previous = 0;
    let mut current = 1;

    for _ in 0..n {
        let temp = previous + current;
        previous = current;
        current = temp;
    }

    current
}

fn fib_recursive(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn main() {
    println!("Enter the number of terms for fibonacci series: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().expect("Pelase enter a valid number");

    println!()
}
