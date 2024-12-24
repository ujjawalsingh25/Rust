// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
    
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("The value of x is: {THREE_HOURS_IN_SECONDS}");

//     const A: u32 = THREE_HOURS_IN_SECONDS / 3600;
//     println!("The value of x is: {A}");

// }


// _________________  Shadowing  _________________________________
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}