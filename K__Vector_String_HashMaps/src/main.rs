#![allow(dead_code, unused_variables, non_snake_case)]

mod a__vectors;
mod b__strings;
mod c__hashmaps;

fn main() {

    println!("\n__________________  Vector Basic  __________________");
    a__vectors::vector_basics();      
    println!("________  Enum And Vector  ___________");
    a__vectors::enum_and_vector();      

    println!("\n__________________  String Basic  __________________");
    b__strings::string_basics();      
    
    println!("\n\n__________________  Hash Basic  __________________");
    c__hashmaps::hashmaps_basics();

}