mod a__struct;
mod b__struct_example;
mod c__method_syntax;

fn main() {

    println!("\n__________________  Struct Basic  __________________");
    a__struct::struct_basic();         // Imported from other file....

    println!("\n__________________  Struct Example  __________________");
    b__struct_example::struct_example();         // Imported from other file....
    
    println!("\n__________________  Method Syntax  __________________");
    c__method_syntax::method_syntax();          // Imported from other file....

}