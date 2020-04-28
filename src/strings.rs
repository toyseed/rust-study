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
    let mut n_src = String::from(src);

    n_src.push_str(" new string");

    println!("{}", n_src);

}

#[test]
fn test_string_length() {
    let say_hi = "안녕?";

    println!("length of say_hi: {}", say_hi.len());
    println!("length of chars of say_hi: {:?}", say_hi.chars());

    let h = &say_hi[0..3];
    println!("slice first: {}", h);

}

#[test]
fn test_type_check() {
    let int_str = String::from("123");
    let float_str = String::from("10.23");
    let _test_str = String::from("abck23kj3.223");

    let int_to_int = int_str.trim().parse::<i32>().unwrap();
    let int_to_flat = int_str.trim().parse::<f64>().unwrap();

    println!("int_to_int {}", int_to_int);
    println!("intToFloat {}", int_to_flat);

    // let floatToInt = float_str.trim().parse::<i32>().unwrap();    // err
    let float_to_float = float_str.trim().parse::<f64>().unwrap();
    println!("float_to_float {}", float_to_float);

}