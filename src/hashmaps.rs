use std::collections::HashMap;

#[test]
fn test_hashma_key() {
    let mut scores = HashMap::<&str, i32>::new();

    let key = &String::from("key")[..];
    // let key = &key[..];

    scores.insert("key", 10);
    scores.insert(key, 11);

    println!("{}", scores.get(key).unwrap());

    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }
}