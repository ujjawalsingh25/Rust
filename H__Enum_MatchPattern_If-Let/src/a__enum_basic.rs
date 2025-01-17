// --------------------------------  ENUM with Struct and Struct Impl  -------------------------------
// _________________________________________________________________________
// #[derive(Debug)]                                                     //  |
// enum IpAddrKind { V4, V6 }                                           //  |
// struct IpAddress {                                                   //  |
//     address: String,                                                 //  |
//     kind: IpAddrKind,                                                //  |
// }                                                                    //  |         
// impl IpAddress {                                                     //  |
//     fn new(address: &str) -> Self {                                  //  |
//         Self {                                                       //  |
//             address: address.to_string(),                            //  |
//             kind: IpAddrKind::V4,                                    //  |
//         }                                                            //  |
//     }                                                                //  |  
// }                                                                    //  |
// fn  route(ip: IpAddress) {                                           //  |
//     println!("Routing IP: {} \nKind: {:?}", ip.address, ip.kind);    //  |
// }                                                                    //  |
// _________________________________________________________________________|

// --------------------------------  ENUM Basics   ----------------------------------
#[derive(Debug)]
enum IpAddrKind { 
    V4(u8, u8, u8, u8), 
    V6(String), 
}
fn  route(ip: IpAddrKind) {                                           
    println!("Routing IP: {:?}", ip);    
} 

pub fn enum_basic() {
    let my_ip = IpAddrKind::V4(129, 34, 23, 30);
    let loopback = IpAddrKind::V6(String::from("127.0.0.0"));
    route(my_ip);
    route(loopback);
}
// ---------------------------------------------------------------------------------------


// --------------------------------  Implementation in ENUM   -------------------------------
// _________________________________________________________________________________________________________________________
#[derive(Debug)]                                                                                                        //  |
enum Message {                                                                                                          //  |
    Quit,                                                                                                               //  |
    Move { x: i32, y: i32 },                                                                                            //  |
    Write(String),                                                                                                      //  |
    ChangeColor(i32, i32, i32),                                                                                         //  |
}                                                                                                                       //  |
impl Message {                                                                                                          //  |
    fn call(& mut self) {                                                                                               //  |
        // let Message::Write(ref mut text) = self;        //******// self is an immutable reference (&self),               | 
        // text.push_str(" world");                        //******// so you cannot mutate text by taking                   | 
        // println!("Name: {}", text);                     //******// a mutable reference (ref mut text).                   |
        //                                                                                                                  |
        println!("{:?}", self);                                      //**//  Instead, you need to match  //**//             | 
        if let Message::Write(ref mut text) = self {    //**// it properly using "if let" or "match". //**//                |
        text.push_str(" Singh");                                                                                        //  |
        println!("Name: {}", text);                                                                                     //  |
    }                                                                                                                   //  |
}                                                                                                                       //  |
}                                                                                                                       //  |
pub fn impl_in_enum() {                                                                                                 //  |
    let mut m = Message::Write(String::from("Ujjawal"));                                                                //  |
    m.call();                                                                                                           //  |
}                                                                                                                       //  |
// _________________________________________________________________________________________________________________________|



// --------------------------------  Option ENUM   -------------------------------
// ___________________________________________________
// enum Option<T> {
//     None,
//     Some(T),
// }

pub fn option_enum() {
    let op1 = Option::Some(10);
    let op2: Option<i32> = None;
    let x = 5;
    
    let sum1 = x * op1.unwrap();    
    let sum2 = x * op2.unwrap_or(0);   
    println!("Sum1: {}", sum1); 
    println!("Sum2: {}", sum2); 
}
// ___________________________________________________