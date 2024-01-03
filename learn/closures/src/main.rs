fn main() {
    let list = vec![1,2,3,4,5,6,7,8];
    println!("Before defining closure: {:?}", list);

    let only_borrows  = || println!("From closures: {:?}", list);


    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
