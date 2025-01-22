// The main aim of lifetimes is to prevent dangling references, 
// which cause a program to reference data other than the data it’s intended to reference. 

pub fn lifetimes_basic() {
    let x = 5;            // ----------+-- 'b
                               //           |
    let r = &x;          // --+-- 'a  |
                               //   |       |
    println!("r: {r}");        //   |       |
                               // --+       |
}                              // ----------+


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// -----------------------------------  Rules of Lifetime  -----------------------------------
// First Rule
// The first rule is that the compiler assigns a lifetime parameter to each parameter 
// that’s a reference. In other words, a function with one parameter gets one lifetime 
// parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate 
// lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// Second Rule
// The second rule is that, if there is exactly one input lifetime parameter, 
// that lifetime is assigned to all output lifetime parameters: 
// fn foo<'a>(x: &'a i32) -> &'a i32.

// Third Rule
// The third rule is that, if there are multiple input lifetime parameters, 
// but one of them is &self or &mut self because this is a method, the lifetime 
// of self is assigned to all output lifetime parameters. This third rule makes 
// methods much nicer to read and write because fewer symbols are necessary.
// ---------------------------------------------------------------------------------------------