struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (32, 52);

    let rect2 = Rectangle {
        width: 80,
        height: 60,
    };

    println!(
        "The area of trianlge using {} struct ",
        area_using_struct(&rect2)
    );

    println!(
        "The area of rectangle is {} square pixles ",
        area(width1, height1)
    );

    println!(
        "The area using tupble way {} sqare pixels ",
        area_using_tuple(rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_using_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
