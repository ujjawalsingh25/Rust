struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,      // if key and value are same then only one can be used 
        email,                      // as here only "email" used not {email: email},
        active: true,
        sign_in_count: 25,
    }
}

pub fn struct_basic() {
    let mut user1: User = User {
        email: String::from("ujjawalsingh@gmail.com"),
        username: String::from("Ujjawal Singh"),
        sign_in_count: 25,
        active: true,
    };
    user1.username = String::from("Chak");
    user1.username.push_str(" Bairia");
    println!("User1 Username: {}", user1.username);
    
    let s1: String = user1.username;                // Here 's1' have address of String "Chak Bairia"
    user1.username = String::from("Patna");         // And "username" have now new address of String "Patna"
    println!("S1: {}", s1);                         // Hence, Ownership rule Valid of  
    println!("New User1 Username: {}", user1.username);       // having single owner of each variable
    
    let user2: User = build_user(
        String::from("Ujjawal Kumar"),
        String::from("ujjawalkumar@gmail.com"),
    );
    println!("User2 Username: {}", user2.username);

    let user3: User = User {
        active: false,          // changes just active value rest all
        ..user2                 //  value of 'user2' is moved to 'user3' 
    };
    // println!("User2 Username: {}", user2.username);      // ERROR  ->> Since value moved
    println!("User3 Active: {}", user3.active);
    println!("User3 Username: {}", user3.username);


    let red = Color(25, 28, 12);       // struct of tuples
    set_bg_color(red);
    let point = Point(20, 30, 50);
    move_point(point); 
}

struct Color(u8, u8, u8);      // struct of tuples
struct Point(u8, u8, u8);     // struct of tuples

fn set_bg_color(color: Color) {
    println!("BG Color:  R-> {},  G-> {},  B-> {}",color.0, color.1, color.2)
}
fn move_point(point: Point) {
    println!("Point at:  x-> {},  y-> {},  z-> {}",point.0, point.1, point.2)
}