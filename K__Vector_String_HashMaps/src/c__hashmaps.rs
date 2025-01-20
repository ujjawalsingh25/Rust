use std::collections::HashMap;

pub fn hashmaps_basics() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);    
    println!("HashMap: {:?}", scores);
    
    // ----------  Overwritting, Adding Value Only If a Key not exist  -----------
    scores.insert(String::from("blue"), 25);      // Overwrites the value
    println!("HashMap: {:?}", scores);                  // Overwrites the value

    scores.entry(String::from("green")).or_insert(12);  // Add Value only when Key not exist
    println!("HashMap: {:?}", scores);            // Adding Value Only If a Key isn’t Present
    // ------------------------------------------------------------

    // -------  Iterating in HashMaps  ----------
    for (key, value) in &scores {
        println!("{:?}--> {:?}", key, value);
    }
    // -----------------------------------------------
    

    // -------  Value using "get" on Key  ----------
    let score = scores.get(&String::from("red"))
                            .copied().unwrap_or(0);     // just (default: 0) not (default: &0) 
                            // .unwrap_or(&0);    // Since used ".copied()" so need to pass reference
    println!("Red Score: {}", score);
    // ---------------------------------------------


    // -------------------  Hash Maps and Ownership  ---------------------------
    let game = String::from("Sports");
    let value = String::from("Volleyball");

    let mut map = HashMap::new();
    map.insert(game, value);            // map takes the ownership so game and value will be invalid later
    println!("Mapping Sport: {:?}", map);
    // map.insert(game, value);       // "game" and "value" are invalid at this point
    // ---------------------------------------------------------


    // -------------------  Updating a Value Based on the Old Value  ---------------------------
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}"); 
    for (key, value) in &map {
        println!("{:?}--> {:?}", key, value);
    }
    // ---------------------------------------------------------
    
}