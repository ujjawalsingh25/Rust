#![allow(dead_code, unused_variables, non_snake_case)]

pub fn references() {
    let mut s = String::from("Ujjawal");

    let len = calculate_length(&mut s);

    println!("The length of '{s}' is {len}");


    cannot_have_multiple_mutable_references();

    dangle_references();
}
fn calculate_length(s: &mut String) -> usize {
    s.push_str(" Singh");
    let length = s.len();   // len() returns the length of a String
    length
}



fn cannot_have_multiple_mutable_references() {

    // -----------------------------------------------------------------------------------------------------
    // -----------------------------------------------------------------------------------------
    let mut s = String::from("hello");

    let r1 = &mut s;        
    // let r2 = &mut s;         error says that this code is invalid because 
                        // we cannot borrow s as mutable more than once at a time. 
                        // The first mutable borrow is in r1 and must last until 
                        // it’s used in the println!, but between the creation of 
                        // that mutable reference and its usage, we tried to create 
                        // another mutable reference in r2 that borrows the same data as r1.

        // *****  r2 can't even takes simple referrence (let r2 = &s) because r1 is mut and 
        // possibly can change 's' value before 'r2' used it ******

    // println!("{}, {}", r1, r2);
    // -----------------------------------------------------------------------------------------
    // -----------------------------------------------------------------------------------------------------


    // -----------------------------------------------------------------------------------------------------
    // -----------------------------------------------------------------------------------------
    let mut s2 = String::from("hello");
    
    {
        let r3 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
    let r4 = &mut s2;   // Now this time it is OK to have multiple mutable references as
    //                                  'r3' scope ends before 'r4' starts to takes the memory or execute
    // -----------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------------------
    

    // ----------------------------------------------------------------------------------------------------
    // -----------------------------------------------------------------------------------------
    // let mut s = String::from("hello");
    
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //  We also cannot have a mutable reference while we have an immutable one to the same value.
    // However, multiple immutable references are allowed because one variable do not affects 
    // other variable data as bothjust reading and cannot change the value
    
    // -----------------------------------------------------------------------
    // -----------------------------------------------------------------------
    //  **************** 
    //  ***** ---->>>>> Note that a reference’s scope starts from where it is introduced 
    // and continues through the last time that reference is used. *****<<<<<-------
    //  *********************

    // let mut s = String::from("hello");
    
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{r1} and {r2}");
    // // variables r1 and r2 will not be used after this point
    
    // let r3 = &mut s; // no problem    // ****** However, variable scope of 'r1' and 'r2' remain
    // println!("{r3}");                // BUT 'Reference Scope' over so can 'mut' multiple references with "NO ERROR" 
    
    // -----------------------------------------------------------------------------------------
    // ----------------------------------------------------------------------------------------------------
    
}



fn dangle_references() {
    // let reference_to_nothing = dangle();             // ERROR 
    let reference_to_nothing = no_dangle();
}

// fn dangle() -> &String {                     // dangle returns a reference to a String
//     let s = String::from("hello");          
//     &s                                     // we return a reference to the String, s
// }                                         // Here, s goes out of scope, and is dropped. Its memory goes away.
//                 So, cause ERROR --> it return a reference / pointer to empty memory which is dropped after scope over

fn no_dangle() -> String {
    let s = String::from("Ujjawal");
    s                   // Therefor, we pass the 's' String or the "OWNERSHIP" so pointer donot point to null
            // But to a moved value as Ownership given back to varible calling function i.e(reference_to_nothing)
}