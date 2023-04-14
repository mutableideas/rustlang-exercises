#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }

    fn can_fit(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle {
        height: 30,
        width: 30
    };

    let smaller_rect = Rectangle {
        height: 15,
        width: 25
    };

    let area = get_rectangle_area(&rect);
    println!("The area of the rectange is: {}", area);
    println!("The rect properties {:#?}", rect);
    println!("The area from struct {}", rect.area());
    println!("The rectange is a square: {}", rect.is_square());
    println!("Rectangle can fit in rect: {}", rect.can_fit(&smaller_rect));
}

// Borrow rather than own the rect
fn get_rectangle_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

