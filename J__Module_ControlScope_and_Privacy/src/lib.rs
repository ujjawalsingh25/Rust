#![allow(dead_code, unused_variables, non_snake_case)]
// _____________________________________________________________________
// crate J__Module_ControlScope_and_Privacy                         //  |
// ├── mod back_of_house: pub(crate)                                //  |
// │   └── struct Breakfast: pub                                    //  |
// │       └── fn summer: pub                                       //  |
// ├── fn eat_at_resturant: pub(crate)                              //  |
// └── mod front_of_house: pub(crate)                               //  |
//     ├── mod hosting: pub                                         //  |
//     │   ├── fn add_to_waitlist: pub                              //  |
//     │   └── fn seat_at_table: pub                                //  |
//     └── mod serving: pub(self)                                   //  |
//         ├── fn serve_order: pub(self)                            //  |
//         ├── fn take_order: pub(self)                             //  |
//         └── fn take_payment: pub(self)                           //  |
// -----------------------------------------------                  //  |
// mod front_of_house {                                             //  |
//     pub mod hosting {                                            //  |
//         pub fn add_to_waitlist() {}                              //  |
//         pub fn seat_at_table() {}                                //  |
//     }                                                            //  |
//                                                                  //  |
//     mod serving {                                                //  |
//         fn take_order() {}                                       //  |
//         fn serve_order() {}                                      //  |
//         fn take_payment() {}                                     //  |
//     }                                                            //  |
// }                                                                //  |
// mod back_of_house {                                              //  |
//     pub struct Breakfast {                                       //  |
//         pub toast: String,                                       //  |
//         seasonal_fruit: String,                                  //  |
//     }                                                            //  |
//     impl Breakfast {                                             //  |
//         pub fn summer(toast: &str) -> Breakfast {                //  |
//             Breakfast {                                          //  |
//                 toast: String::from(toast),                      //  |
//                 seasonal_fruit: String::from("peaches"),         //  |
//             }                                                    //  |
//         }                                                        //  |
//     }                                                            //  |
// }                                                                //  |
// _____________________________________________________________________|

mod front_of_house;
mod back_of_house;
mod customer;

use front_of_house::hosting::add_to_waitlist;
fn eat_at_resturant() {
    // crate::front_of_house::hosting::add_to_waitlist();       // ERROR -->> since front_of_house in not public
    // front_of_house::hosting::add_to_waitlist();        // can access through inner path but not through "root" path
    add_to_waitlist();                                  // just call with "use at above"

    // let break_fast = back_of_house::Breakfast {
    //     toast: String::from("Wheat")
    // }
    let break_fast = back_of_house::Breakfast::summer("Wheat");

}     

// ----------------------  Providing New Names with the as Keyword  ---------------------------
// use std::fmt;
// use std::io;
// use std::ios as IosResult;

// fn function1() -> fmt::Result {
//     // --snip--
// }
// fn function2() -> io::Result<()> {
//     // --snip--
// }
// fn function2() -> IosResult<()> {
//     // --snip--
// }

