pub fn vector_basics() {
    // let mut arr = Vec::new();
    // arr.push(1);
    // arr.push(2);
    let mut arr: Vec<i32> = vec![1, 2, 3];
    arr.push(4);
    println!("Vector: {:?}", arr);
    println!("arr[1]: {:?}", arr[1]);
    println!("arr[2] with get: {:?}", arr.get(2));
    println!("  -->> It return as 'Option' with Some and Error 
            so that 'Out of Bound' cases can handle and don't panic the code");
    
    println!("arr[12] with get and 'unwrap_or': {:?}", arr.get(13).unwrap_or(&-1));
    // OR -----------
    let third_val = match arr.get(3) {
        Some(value) => value,
        None => {
            println!("Out of bound");
            & -1
        },
    };
    println!("arr[3]: {}", third_val);


    // -----------------------------------------
    // let arr = arr;
    // arr.push(3);

    // let mut arr2 = vec![1, 2, 3, 4, 5];
    // let arr2_first = &arr2[0];
    // v.push(6);                   // ERROR // "Borrow Checker" don't allow to modify 
    //                                  // Since Slice Reference of arr2 is still valid
    // -----------------------------------------
    

    // ------------------  Iterating Over the Values in a Vector  -------------------
    let arr3 = vec![3, 7, 4];
    for arrIter in &arr3 {
        print!("{arrIter} ");
    }println!("\n---- Added 2 ----");
    
    let mut arr4 = vec![3, 7, 4];
    for arrIter in &mut arr4 {
        *arrIter += 2;              // ******// *arrIter -->> since "arrIter" is memory address so needed "*"
        // *arrIter = *arrIter + 2;              // OR // *arrIter -->> since "arrIter" is memory address
        //                                  // "&"  -->> "Reference Operator"
        //                                  // "*"  -->> "DeReference Operator"
        print!("{arrIter} ");
    }println!();
    // ----------------------------------------------------------------------------------

}




// ------------------  Using an Enum to Store Multiple Types  -------------------
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn enum_and_vector() {
    let cells = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("Red")),
        SpreadsheetCell::Float(10.30),
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Blue")),
    ];
    println!("Enum and Vector: {:#?}", cells);

    let index = 1;
    match cells.get(index) {
        Some(SpreadsheetCell::Int(val)) => println!("Int Val: {}", val),
        Some(_) => println!("Not an Int Value"),
        None => println!("No Value"),       
    }
}
// ----------------------------------------------------------------------------------  