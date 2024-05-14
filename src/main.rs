// Generics
// These serve as Abstract standins for concrete data types and other properties.
// These can be used with structs, functions, methods, and more.
// Using Generics, you can make the type of any given field whatever type you wish, plus, you can
// define multiple types.  You define it as a generic using the <T> syntax after you give the
// struct a name. If you only define the struct with one generic type <T>, if you try to declare
// any other field in the struct as a different type, you will get a compile error because once the
// first field gets a type <T> assigned to it, it tells the program that Rectanle is now of type <T> (int32, f64, whatever you assign it as)
// However, if yo assign the struct with multiple generics, you can define the width to whatever
// type you wish, and the height to whatever type you wish:
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        return &self.width;
    }
}
// By not including the generic types after the impl keyword, this is telling the compiler that
// this method will only apply to rectangles that are of a specific concrete type:
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        return 2 * self.width + 2 * self.height;
    }
}
// Generics can also be used to define functions:
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn main() {
    let rect = Rectangle {
        width: 4,
        height: 4.5,
    };
    println!(
        "This rectangle has a width of {} and a height of {}",
        rect.width, rect.height
    );

    println!("The width of the rectangle is {}", rect.get_width());
    let rect2 = Rectangle {
        width: 4u8,
        height: 5u8,
    };
    println!(
        "The perimeter of the second rectangle is {}",
        rect2.get_perimeter()
    );

    let rect3 = Rectangle {
        width: 5u8,
        ..rect2
    };

    let perimeter1 = rect2.get_perimeter();
    let perimeter2 = rect3.get_perimeter();

    println!(
        "The largest perimeter out of the two rectangles is {}",
        get_biggest(perimeter1, perimeter2)
    );
}
