#![allow(dead_code, unused_variables, non_snake_case)]
// _________________________________________________________________________________________
// mod database {                                                                           |
//     pub enum Status {                                                                    |
//         Connected,                                                                       |
//         Interrupted,                                                                     | 
//     }                                                                                    |
//                                                                                          |                        
//     pub fn connect_to_database() -> Status {                                             |
//         //connect to db..                                                                |
//         Status::Connected                                                                |
//     }                                                                                    |
//     pub fn get_user() {                                                                  |
//         // fetch the user from db and return                                             |
//     }                                                                                    |
// }                                                                                        |
//                                                                                          |     
// pub mod auth_utils {                                                                     |
//                                                                                          |
//     pub mod models {                                                                     |
//         pub struct Credentials {                                                         |
//             pub username: String,                                                        |
//             pub password: String,                                                        |
//         }                                                                                |
//     }                                                                                    |
//     pub fn login(cred: models::Credentials) {                                            |
//         // try to login                     // since can't access database directly      |
//         // super::database::get_user()      //  so calls from "super" class              |
//         crate::database::get_user()         // get database from "root"                  |
//     }                                                                                    |
// }                                                                                        |
//                                                                                          |
// pub fn authenticate(cred: auth_utils::models::Credentials) {                             |
//     if let database::Status::Connected = database::connect_to_database() {               |
//         auth_utils::login(cred);                                                         |
//     }                                                                                    |
// }                                                                                        |
// _________________________________________________________________________________________|



// -----  The above code is now organised in multiple file str -------
// _____________________________________________________________________
#![allow(dead_code, unused_variables)]                              //  |
//                                                                  //  |
mod util;                                                           //  |
// src/util.rs        -->> "Utils" search in                        //  |
// src/util/mod.rs      -->> these file structure                   //  |
//                                                                  //  |
mod database;                                                       //  |
//                                                                  //  |
pub mod auth_utils;                                                 //  |
//                                                                  //  |
use auth_utils::login;                                              //  |
use database::{connect_to_database, Status};                        //  |
//                                                                  //  |
pub fn authenticate(cred: auth_utils::models::Credentials) {        //  |
    if let Status::Connected = connect_to_database() {              //  |
        login(cred);                                                //  |
    }                                                               //  |   
}                                                                   //  |
// _____________________________________________________________________|