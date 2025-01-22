#![allow(dead_code, unused_variables, non_snake_case)]

mod a__generic_types;
mod b__traits;
mod c__lifetimes;

fn main() {

    println!("\n__________________  Generic Types  __________________");
    a__generic_types::gerericTypes();      
    println!("________  Gereric with Struct  ___________");
    a__generic_types::gereric_with_Struct();

    println!("\n__________________  Traits  __________________");
    b__traits::traits_basic();      
    
    println!("\n__________________  Lifetimes  __________________");
    c__lifetimes::lifetimes_basic();      
    
}