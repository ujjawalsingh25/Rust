fn main() {
    let y = is_even(4);
    println!("Value of y: {y}");

    // let mut x: i32 = 0;
    // if y {
    //     x = 10;
    // } else {
    //     x = 20;
    // }
    let x = if y {15} else {25};
    println!("Value of x: {x}");

    // ______________  Loop  ____________________
    let mut counter = 0;
    
    let result = 'counting_up_loop: loop {
        counter += 1;
        println!("Counter1: {counter}");
        
        if counter == 3 {
            println!("Counter(loop 1): {counter}");
            // break counter * 5;       // return value i.e 50 (10*5) 
            break 25;       // return value 25
        }
        loop {                  // first checks for 10 then inside this loop and exist only at 20 and return 50
            counter += 1;
            println!("Counter(loop 2): {counter}");
            if counter == 5 {
                break 'counting_up_loop 50;
            }
        }
    };
    
    println!("Counter: {result}");
    
    // ______________  For  ____________________
    let arr = [1, 3, 5, 2, 7];
    for i in arr {
        println!("arr[{i}]: {}", i);    // i ->> arr[i]
    }

    for i in 1..3 {
        println!("For using for range: {i}");
    }
    for i in (1..=3).rev() {
        println!("For using for range in Reverse: {i}");
    }
}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    return false;
}