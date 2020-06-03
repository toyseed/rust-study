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

#[test]
fn test_update_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let mut count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    for (_, (key, value)) in map.iter().enumerate() {
        println!("key: {}, value: {}", key, value);
    }

    for (key, value) in map.iter() {
        println!("key: {}, value: {}", key, value);
    }
}