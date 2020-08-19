#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.height > other_rect.height && self.width > other_rect.width
    }
}

impl Rectangle {
    fn square(length: f32) -> Rectangle {
        Rectangle {
            width: length,
            height: length,
        }
    }
    
    fn golden_ratio(length: f32) -> Rectangle {
        Rectangle {
            width: length * 1.61,
            height: length,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30 as f32,
        height: 50 as f32,
    };
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square inches", rect1.area());

    let rect2 = Rectangle {
        width: 10 as f32,
        height: 40 as f32,
    };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 60 as f32,
        height: 45 as f32,
    };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect3));

    let golden_rect = Rectangle::golden_ratio(10 as f32);

    println!("The width of golden_rect is {}", golden_rect.width);

    let square = Rectangle::square(5 as f32);
    assert_eq!(square.width, square.height);
}
