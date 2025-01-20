pub fn string_basics() {
    
    // --------------------  Basics  -------------------------    
    let mut name = String::from("Ujjawal");
    name.push_str(" Kumar");
    name.push(' ');     
    let title = String::from("Singh");
    let name2 = name + &title;              // takes "title" copies to "name" and then give ownership to "name2"        
    println!("Name: {}", name2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");      // "format! macro" returns a String with the contents. 
    //            // It uses references so that this call doesn’t take ownership of any of its parameters.
    println!("{s}");
    // -------------------------------------------------------------------------------


    // --------------------  Indexing into Strings  -------------------------    
    // let hello = "Здравствуйте";      
    // If you were asked how long the string is, you might say 12. In fact, Rust’s answer is 24: 
    // that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode 
    // scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s 
    // bytes will not always correlate to a valid Unicode scalar value
    
    // let answer = &hello[0];      // ERROR //
    // You already know that answer will not be З, the first letter. When encoded in UTF-8, 
    // the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, 
    // but 208 is not a valid character on its own. Returning 208 is likely not what a user would want 
    // if they asked for the first letter of this string; however, that’s the only data that 
    // Rust has at byte index 0. Users generally don’t want the byte value returned, even if 
    // the string contains only Latin letters: if &"hello"[0] were valid code that returned 
    // the byte value, it would return 104, not h.
    // The answer, then, is that to avoid returning an unexpected value and 
    // causing bugs that might not be discovered immediately
    // ----------------------------------------------------------------
    // --------  Methods for Iterating Over Strings  ---------- 
    println!("---------  Iterating in strings  ---------");
    print!("Printing String 'Зд' by iterating indexes: ");
    for c in "Зд".chars() {
        print!("{c} ");
    }
    // -------------------------------------------------------------------------------
    
    
    // --------  Bytes, Scalar Values and Grapheme Clusters Representation  ---------- 
    let str_rep = String::from("नमस्ते");
    // Bytes "नमस्ते"
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    
    // Scalar "नमस्ते"
    // ['न', 'म', 'स', '्', 'त', 'े']
    
    // Grapheme Clusters "नमस्ते"
    // ["न", "म", "स्", "ते"]    
    // -------------------------------------------------------------------------------
    
}