struct User {
    email: String
}
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result = &list[0];
    for listIter in list {
        if listIter > result {
            result = listIter;
        }
    } 
    result
}   
pub fn gerericTypes() {
    let number_list = vec![1, 34, 53, 24, 19];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let users = vec![
        User {
            email: String::from("ujjawal@gmail.com")
        },
        User {
            email: String::from("singh@gmail.com")
        }
    ];

    let result = largest(&number_list);
    println!("The largest number is {result}");
    let result = largest(&char_list);
    println!("The largest char is {result}");
}


// ------------------------------------------------------------------------------------
struct Axis<T> {
    a: T,
    b: T,
}
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self {x, y}
    }
    fn mixup<X, Y>(self, point: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}
impl Point<f64, f64> {
    fn calculate_dist(&self) -> f64 {
        4.0
    }
}
pub fn gereric_with_Struct() {
    let int_axis = Axis { a: 5, b: 10 };
    let float_axis = Axis { a: 1.0, b: 4.0 };
    
    // let diff_type = Axis { a: 1, b: 4.0 };          // ERROR  -->> Both should be of same type
    let diff_type = Point { x: 1, y: 4.0 };    // NO ERROR  -->> Since 2 generics

    let point0 = Point::new(10, 1.3);
    let point1 = Point::new(5.2, 5);

    let point2 = Point::new(2.5, 5.2);
    // point1.calculate_dist();        //  ERROR  -->> point2 have "i32, f64" but fn "caldist" need "f64, f64"
    point2.calculate_dist();

    let point3 = point0.mixup(point1);
    println!("Point3: {:?}", point3);

}