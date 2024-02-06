fn factorial( n: u64 ) -> u64 {

    if n <= 1 {
        return 1;
}
return n * factorial(n-1);
}

fn main() {
    let number = 5;

    let number = factorial ( number);

    println!("the factorail of 5 is {} ", number);
}

