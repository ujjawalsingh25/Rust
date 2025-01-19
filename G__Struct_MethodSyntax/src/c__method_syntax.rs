#![allow(dead_code, unused_variables, non_snake_case)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {                        // implementation
    fn calculate_area(&self) -> u32 {
        self.height * self.width
    }
    fn calculate_area_with_incValue(& mut self) -> u32 {      // "& mut self" -->> it takes as & mut self
        (self.height + 3) * self.width
    }
}
impl Rectangle {                                    // Multiple Implementation "impl" Blocks POSSIBLE
    fn can_hold_other_rect(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(side: u32) -> Self {    // "Self" is termed as -->> fn square(size: u32) -> Rectangle
        Rectangle {
            width: side,
            height: side,
        }
    } 
}

pub fn method_syntax() {
    let rect1 = Rectangle {
        width: 25,
        height: 20,
    };
    println!("Area rect1: {}", rect1.calculate_area());
    
    let mut rect2 = Rectangle {width: 10, height: 12};
    println!("Area rect2: {}", rect2.calculate_area_with_incValue());
    
    println!("Can rect1 hold rect2?: {}", rect1.can_hold_other_rect(&rect2));
    println!("Can rect2 hold rect1?: {}", rect2.can_hold_other_rect(&rect1));

    let sq = Rectangle::square(10);
    println!("{:?}", sq);
}