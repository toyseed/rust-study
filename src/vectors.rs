#[test]
fn test_lock() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("first {:?}", first);
    v.push(6);
    println!("v {:?}", v);
}

#[test]
fn test_mut_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    let first = &v[0];
    println!("first : {}", first);

    for i in &v {
        println!("{}", i);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


#[test]
fn iter_vector_with_enum() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for sc in &row {
        match sc {
            SpreadsheetCell::Int(i) => println!("int: {}", i),
            SpreadsheetCell::Float(f) => println!("float: {}", f),
            SpreadsheetCell::Text(s) => println!("text: {}", s),
        }
    }
}

