fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Not divide by 0"));
    }
    Ok(x / y)
}

use std::fs::File;

pub fn RecoverableErrors() {
    let r = match divide(5, 0) {
        Ok(num) => num,
        Err(err) => {
            println!("Err: {}", err);
            -1
        }
    };
    println!("Divide: {}", r);


    // let greeting_file_result = File::open("hello.txt")?;   // "?" -->> idealy return if file exist
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Something went wrong: {error:?}");
            }
        },
    };

}