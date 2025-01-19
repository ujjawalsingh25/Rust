#![allow(dead_code, unused_variables, non_snake_case)]

fn main() {

    // -------------------------------------  Scaler Data Types---------------------------------------------------
    // -----------------------------------------------------------------------------------------------------------
    
    let a: i8 = -5;          // Signed so can be both positive and negative  
    let b: u8 = 5;          // Unsigned so always positive
    let aa = -25i8;   //OR -25_i8  // aa = -25    // varible type can be given in all ways  
    
    let c: f32 = 3.1234567;     // f32 can take less precisions upto decimal 7 (SIngle Precision)
    let _cf: f64 = 3.123456789123456;        // f64 can take more precisions (Double Precision)
    
    let d: char = 'D';              // ' ' (Single Qoutes) -->> are for string
    let st: &str = "Ujjawal";       // " " (Double Qoutes) -->> are for string
    let e: bool = true;
    println!("Value of   a --> {}   and   aa --> {}", a, aa);      // in both way 
    println!("Value of   b --> {b}    and   c --> {c}");           // we can print variables
    println!("Value of   d --> {d}    and  e --> {}", e);
    println!("Value of STring --> {st}");
    
    let binary_num = 0b1010;        // '0b' converts number to binary
    println!("Value of Number in Binary -->> {binary_num}  -> '0b' converts number to binary");
    let byte_char = b'A';        // '0b' converts number to binary
    println!("Value of Char in Byte -->> {byte_char}  -> 'b' converts char to byte");
    
    let truncated: i32 = 5 / 2;       // Since explicit i32 so result will be truncate (2.5 --> 2)
    println!("Value of Truncated value -->> {truncated} -> Since, i32 so result will be truncate (2.5 --> 2)");
    let floatt: f32 = 5_f32 / 2.0;   // Since, f32 so need other data types to be float
    println!("Value of Truncated valuess -->> {floatt} -> Since, f32 so need other data types to be float");
    let under_scored = 1_00_00_000;        // Underscored will be ignored to increase readibility.
    println!("Value of under_scored -->> {under_scored}  -> UnderScores are ignored to incr. readibility");
    
    // ______________________________________  Integer Overflow  ___________________________________________
    // u --> unsigned;  i --> signed                                                                        |
    // -(2n - 1) to 2n-1 - 1                                                                                |
    // i8 range = -128 to 127      --> n=8                                                                  |
    // u8 range = 0 to 255         --> n=8                                                                  |
    //                                                                                                      |
    // let g: u8 = 256;        // "Integer Overflow  ->> Value out of range(0-255)"                         | 
    // Rust includes checks for integer overflow that cause your program to panic at runtime if             |
    // this behavior occurs. Rust uses the term panicking when a program exits with an error.               |
    //                                                                                                      |
    // When you’re compiling in release mode with the --release flag, Rust does not include checks          |
    // for integer overflow that cause panics. Instead, if overflow occurs,                                 |
    // Rust performs two’s complement wrapping. In short, values greater than the maximum value             |
    // the type can hold “wrap around” to the minimum of the values the type can hold.                      |
    // In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.                    |    
    //                                                                                                      |
    // let g: u8 = 256;   -->> So, at "cargo run --release"  the output will be 1 (as cross max by 1)       |
    // let g: u8 = 257;   -->> So, at "cargo run --release"  the output will be 2 (as cross max by 2)       |
    // _____________________________________________________________________________________________________|
    
    // -----------------------------------------------------------------------------------------------------------
    // -----------------------------------------------------------------------------------------------------------
    
    // --------------------  Compound Data Types ( TUPLES and ARRAY )  -------------------------------------------
    // -----------------------------------------------------------------------------------------------------------
    // _____________   Tuple (Values can have different type) ________________
    let tuple: (i32, f64, u8, bool) = (25, 6.423, 1, true);
    let (x, _y, _z, _bl) = tuple;                                       
    println!("0th value is: {x}");
    println!("1st value is: {}", tuple.1);
    
    let tuple_unit = ();        // Enpty Return type called "UNIT"
    // ________________________________________________________________________
    
    
    // _____________   Array (Values must have same type)   ___________________
    let arr = [1, 2, 3, 4, 5];
    let arr_with_same_value = [3; 5];    // -->> [3, 3, 3, 3, 3]
    println!("0th value is: {}", arr[0]);
    
}
