#![allow(dead_code, unused_variables, non_snake_case)]

mod a__unrecoverable_errors;
mod b__recoverable_errors;

fn main() {

    println!("\n__________________  Unrecoverable Errors  __________________");
    a__unrecoverable_errors::UnrecoverableErrors();      
    
    println!("\n__________________  Recoverable Errors  __________________");
    b__recoverable_errors::RecoverableErrors();      
    
}