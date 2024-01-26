enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}
enum User {
    name: String,
    age: u8,
    
}
fn main() {
    use crate::Status::{Poor, Rich};

    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The Poor have no money"),
    }
    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
