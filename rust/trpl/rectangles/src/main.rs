#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle.
    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width: u32 = 30;
    let height: u32 = 50;

    println!(
        "The area of the rectangle is {} square pixels (using parameters).",
        area(width, height)
    );

    println!(
        "The area of the rectangle is {} square pixels (using tuples).",
        area_tuples((30, 50))
    );

    let rectangle: Rectangle = Rectangle { width, height };

    // println! does not take ownership of the rectangle, so we can use it again.
    println!("The rectangle is {:#?}", rectangle);
    println!("The rectangle is {:#?}", &rectangle);
    println!("The rectangle is {rectangle:#?}");
    
    println!(
        "The area of the rectangle is {} square pixels (using struct).",
        area_struct(&rectangle)
    );

    println!(
        "The area of the rectangle is {} square pixels (using struct V2).",
        area_struct(&Rectangle { width: 10, height: 40 })
    );

    let dbg_rectangle: Rectangle = Rectangle { width: 10, height: 40 };
    dbg!(&dbg_rectangle);
    println!("The rectangle is {:#?}", dbg_rectangle);

    dbg!(dbg_rectangle);
    // Cannot use the dbg_rectangle after this point, as the value has been moved.
    //println!("The rectangle is {:#?}", dbg_rectangle);

    // Using of methods
    let rectangle_3: Rectangle = Rectangle { width: 10, height: 40 };
    println!(
        "The area of the rectangle_3 is {} square pixels (using struct method).",
        rectangle_3.area()
    );
    println!("Can rectangle hold rectangle_3? {}", rectangle.can_hold(&rectangle_3));

    // Associated function that is not a method.
    let square: Rectangle = Rectangle::square(10);
    println!("The square is {:#?}", square);
}

// Using 2 parameters to pass the dimensions of the rectangle.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Using tuple to pass the dimensions of the rectangle.
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Using our own struct type to pass the dimensions of the rectangle.
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}