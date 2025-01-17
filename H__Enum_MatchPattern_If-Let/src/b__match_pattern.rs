// --------------------------------  Match Pattern  ---------------------------------------
// _________________________________________________________________________________________________________________
#[derive(Debug)]                                                                                                //  |
enum UsState { Alabama, Alaska }                                                                                //  |
//                                                                                                              //  |
#[derive(Debug)]                                                                                                //  |
enum Coin {                                                                                                     //  |
    Penny,                                                                                                      //  |
    Nickel,                                                                                                     //  |
    Quarter(UsState),                                                                                           //  |
}                                                                                                               //  |
//                                                                                                              //  |
fn value_in_cents(coin: Coin) -> u8 {                                                                           //  |
    match coin {                             // (Match is a kind of "SWITCH")  -->>  // Checks as "if-else"     //  |
        Coin::Penny => 12,                  // But need to take all cases (i.e difference in match & switch)    //  |
        Coin::Nickel => {                                                                                       //  |    
            println!("Nickel Coin");             // Since multiple lines so curly braces                        //  |
            15                                  // return 15                                                    //  |
        },                                                // Order Matters a/c to preference                    //  |
        Coin::Quarter(UsState::Alaska) => {              // Eg. if below case runs first its true for           //  |
            println!("At {:?}", UsState::Alaska);      // all Quater so for Alaska it never checks              //  |
            25                                                                                                  //  |
        },                                           // If -->> Alaska then print(statement) and return 25      //  |
        Coin::Quarter(state) => {              // Else  -->> print(below statement) and return  20     //  |
            println!("Another Location: {:?}",state);                                                           //  |
            20                                                                                                  //  |
        }                                                                                                       //  |
    }                                                                                                           //  |
}                                                                                                               //  |
//                                                                                                              //  |    
//                                                                                                              //  |    
pub fn match_pattern() {                                                                                        //  |
    let coin1 = Coin::Penny;                                                                              //  |
    println!("Coin1 Value: {}", value_in_cents(coin1));                                                         //  |
    let coin2 = Coin::Nickel;                                                                             //  |
    println!("Coin2 Value: {}", value_in_cents(coin2));                                                         //  |
    let coin3 = Coin::Quarter(UsState::Alaska);                                                           //  |
    println!("Coin3 Value: {}", value_in_cents(coin3));                                                         //  |
}                                                                                                               //  |
//__________________________________________________________________________________________________________________|




// ---------------------------  Matching with Option<T> --------------------------------
// _________________________________________________________________________________________________
// ----------------------  Matching with Option<T>   --------------                             //  |
fn plus_one(num1: u32, num2: Option<u32>) -> u32 {                                              //  |
    match num2 {                                                                                //  |
        Some(x) => num1 + x,                                                                    //  |
        None => num1,                                                                           //  |
    }                                                                                           //  |
}                                                                                               //  |
pub fn matching_option() {                                                                      //  |
    println!("Add Options with Some: {}", plus_one(37, Some(83)));                              //  |
    println!("Add Options with None: {}", plus_one(37, None));                                  //  |
}                                                                                               //  |
// --------------------------------------------                                                 //  |
//                                                                                              //  |                                    
// ------------------ Catch all Pattern  -----------------                                      //  |
pub fn catch_all_pattern() {                                                                    //  |
    let dice_rolled = 3;                                                                        //  |
    match  dice_rolled {                                                                        //  |
        1 => println!("Smallest Count"),                                                        //  |
        6 => println!("Bigger Count"),                                                          //  |
        // other => println!("Middle Value {}", other),     // Either can write                 //  |
        _ => println!("In Middle"),                     // "other" or "_" both work same        //  |
        // _ => ()                                  // if nothing to do in "OTHER CASES"        //  |     
    }                                                                                           //  |
}                                                                                               //  |
// _________________________________________________________________________________________________|




// ---------------  If Let Concise ControlFlow  --------------
// _________________________________________________________________________
pub fn if_let_concise_controlflow() {                                   //  |
    let dice_rolled = Some(3_u8);       // 3 taken as u8 values         //  |
    if let Some(maxVal) = dice_rolled {                                 //  |
        println!("Max Value: {}", maxVal)                               //  |
    }                                                                   //  |
    let dice_not_rolled: Option<u8> = None;                             //  |      
    if let None = dice_not_rolled {                                     //  |
        println!("Not Rolled");                                         //  |
    }                                                                   //  |
}                                                                       //  |
// _________________________________________________________________________|