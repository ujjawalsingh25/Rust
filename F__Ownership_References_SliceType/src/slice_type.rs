pub fn slice_type() {
    let mut s = String::from("Patna 800007");
                                            // Since, we passed string and get space index and changed the string 
    // let result = space_pos(&s);          // But here both result will not be in sync in below code to print 
    // s.clear();                           // as there will be no string but result gives a 'space at index 5'
       
    
    let result = space_pos_using_slice(&s);     // Since "Slice" so a part of reference is still valid  
                                                        // so can't change value and both results are sync now.
    println!("For string {s} the Result is {}", result.len());
}

fn space_pos(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;        
        }
    }                
    s.len()
}


// -----------  Slice  ------------------------------
fn space_pos_using_slice(s: &String) -> &str {         
    let bytes = s.as_bytes();       

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];     // return slice from start to i(till space)
        }
    }
    &s[..]      // if no space return whole string
}
