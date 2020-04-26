#[test]
fn test_string() {
    let s1 = String::from("hello");
    // let h = s1[0]; // 허용되지 않음. 컴파일 에러.
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 각 글자가 2바이트인 경우임.
}

#[test]
fn test_string_slice() {
    let src= "src string";
    let mut nSrc = String::from(src);

    nSrc.push_str(" new string");

    println!("{}", nSrc);

}

#[test]
fn test_string_length() {
    let sayHi = "안녕?";

    println!("length of sayHi: {}", sayHi.len());
    println!("length of chars of sayHi: {:?}", sayHi.chars());

    let h = &sayHi[0..3];
    println!("slice first: {}", h);

}

#[test]
fn test_type_check() {
    let intStr = String::from("123");
    let floatStr = String::from("10.23");
    let testStr = String::from("abck23kj3.223");

    let intToInt = intStr.trim().parse::<i32>().unwrap();
    let intToFlat = intStr.trim().parse::<f64>().unwrap();

    println!("intToInt {}", intToInt);
    println!("intToFloat {}", intToFlat);

    // let floatToInt = floatStr.trim().parse::<i32>().unwrap();    // err
    let floatToFloat = floatStr.trim().parse::<f64>().unwrap();
    println!("floatToFloat {}", floatToFloat);

}