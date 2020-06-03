use std::borrow::Borrow;

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

    let hello = "Здравствуйте";
    let answer = &hello[0..2];

    println!("{:?}", answer.chars().into_iter().take(3));
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

#[test]
fn test_concat() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);
}

fn substr(s: &str, from: usize, length: usize) -> String {
    s.chars().skip(from).take(length).collect()
}

fn substr_2(s: &str, from: usize, to: usize) -> &str {
    let mut begin: usize = 0;
    let mut begin_byte: usize = 0;
    let mut end: usize = 0;
    let mut end_byte: usize = 0;

    for (i, c) in s.chars().enumerate() {
        if begin < from {
            begin += 1;
            begin_byte += c.len_utf8();

            continue;
        }

        if end < to {
            end += 1;
            end_byte += c.len_utf8();
        }
    }

    // ignore validation check

    &s[begin_byte..(begin_byte + end_byte)]
}

#[test]
fn test_substring() {
    let s = "안a녕b하c세d요?";

    for c in s.chars() {
        println!("char: {}", c);
    }

    println!("{}", s);
    assert_eq!("b하c세", substr(s, 3, 4));
    assert_eq!("b하c세", substr_2(s, 3, 4));

}