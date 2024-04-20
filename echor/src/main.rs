fn main() {
    let mut x = 5;
    let y = 10;
    let z = 15;

    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is not greater than y");
        x = y; // Error: cannot assign twice to immutable variable
    }

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let result = divide(x, z);
    println!("The result is {}", result); // Error: division by zero
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    } else {
        a / b
    }
}fn print_array( a: [i32; 16]) {
    for i in 0..16 {
    println!("{}", a[i]);
    }
}
// create a function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multipy(a: i32, b: i32) -> i32 {
    a * b * 2

}

// print the array of arguments
// $ cargo run 1 2 3
// like giving prnt form 1 to 10

fn print_till_10() {
    for i in 1..12 {
        println!("{}", i);
    }
}