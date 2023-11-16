
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

#[warn(unused_mut)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn check(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn squre(size: u32) -> Rectangle {
        Rectangle{length: size, width: size}
    }
}

fn main() {
    let rectangle = Rectangle{ length: 32, width: 10 };

    println!(
        "Test Print Value {}",
        rectangle.area()
    );

    let rect1 = Rectangle{ length: 10, width: 10};
    let rect2 = Rectangle{ length: 5, width: 5};
    let rect3 = Rectangle{ length: 15, width: 15};

    println!(
        "Test Check Value {} , {}",
        rect1.check(&rect2),
        rect1.check(&rect3)
    );

    let rect4 = Rectangle::squre(40);
    
    println!(
        "TEST Check Value Rect4 {:?}",
        rect4
    );
}
