#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
 
pub fn struct_example() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    let area = calculate_area(&rect);
    
    println!("_________  Using dgb!(value)  _________");
    // let area = dbg!(calculate_area(&rect));      // It returns the value
    // dbg!(rect);    // It takes the Ownership -->>  so Give "Error" for below print statement
    dbg!(&rect);    // It won't takes the Ownership -->>  so "No Error" for below print statement
    
    println!("_________  Using ':?' and ':#?'  _________");
    println!("The area of {:?} is {}", rect, area);      // ":?"  for just printing 
    println!("The area of {:#?} is {}", rect, area);     // ":#?"  for printing in Prettier Way
    //  ":?"  -->> Through this we can print 'struct' or a "Debug Trait"
    //  For ":?" to use we need to derive Debug trait as -->> "#[derive(Debug)]"
    
}