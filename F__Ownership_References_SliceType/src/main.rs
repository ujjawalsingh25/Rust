mod references_and_borrowing;
mod slice_type;

fn main() {
    let num: i32 = 5;                   
    let result: i32 = add(num);          // the value of 'num' is copied not the reference is passed 
    
    println!("Number: {num}");
    println!("Result: {result}");
    println!("Number2: {}", num + 25);      // Since the value was copied so the result don't change the value
    
    let name: String = String::from("Ujjawal Singh");
    takes_ownership(name);               // the function takes the ownership of 'name' as it passed as reference by RUST 
    // println!("Name: {name}");        // and after execution of function the string goes out of scope and 
                                        // get dropped and not exist to memory
    let s: String = gives_ownership();     
    println!("Given_Ownership:  {s}");   // The function return the string so the 'Ownership' of that String is now given to 's'

    let name2 = String::from("Chak Bairia");            // The function takes the 'name2' as parameter and ownership 
    let s2: String = takes_and_gives_back_ownership(name2);    // is given to that function and at end of function it 
    println!("Taken & given ownership: {s2}");                // return the same String so ownership is now to 's2'
    // let name2: String = takes_and_gives_back_ownership(name2);          // ----- OR  ----- 
    // println!("Taken & given ownership: {name2}");       // We can return ownership back to same variable after using 
    
    println!("_______  Tuple code for Ownership  ___________");
    main2();
    println!("________  References and Borrowing  ___________");
    references_and_borrowing::references();         // Imported from other file....
    println!("________  Slice Type  ___________");
    slice_type::slice_type();                      // Imported from other file....
}

fn add(x: i32) -> i32 {
    x + 10
}

fn takes_ownership(s: String) {
    println!("Taken ownership {s}");
}
fn gives_ownership() -> String {
    let str: String = String::from("Given ownership back");
    str         // it returns the string and give back the 'Ownership' from the function
}
fn takes_and_gives_back_ownership(s: String) -> String {
    println!("Inside Ownership taken and given back and ownership taken of string: {s}");
    s
}



// ___________  Takes and return ownership in single function as 'Tuple' _________________
fn main2() {
    let s = String::from("Ujjawal");

    let (s, len) = calculate_length(s);      // Calculate length of String and also returns back the "Ownership"

    println!("The length of '{s}' is {len}");
}

fn calculate_length(s: String) -> (String, usize) {     // Takes the 'Ownership' of String as taken as parameter    
    let length = s.len();           //  and return a Tuple with the (String and Int) means using "Tuple"
                                        // we return backs the "Ownership" and also used the String for the operations
    (s, length)
}
// __________________________________________________________________________________________
