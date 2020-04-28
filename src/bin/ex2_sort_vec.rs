use std::io::Write;

/**
연습문제
정수 또는 실수 또는 문자열을 variants로 갖는 열겨형을 정의하고
그 열거형의 벡터를 선언하여
키보드로 부터 정수, 실수, 문자열을 입력 받아 10개 를 저장하고
정수끼리 구룹지어 소팅하고, 실수끼리 그룹지어 소팅하고, 문자열 끼리 그룹지어 소팅하여
화면에 출력하는 프로그램을 작성해 보세요
**/
#[derive(Debug)]
enum InputTypes {
    Integer(i32),
    Float(f64),
    Text(String),
}

impl InputTypes {
    pub fn get_for(str: &str) -> Option<InputTypes> {
        if str.len() == 0 {
            return None;
        }

        let mut has_text = false;
        let mut has_num = false;
        let mut dot_count= 0;

        for c in str.chars() {
            if c == '.' {
                dot_count += 1;
            } else if !c.is_numeric() {
                has_text = true;
                break;
            } else {
                has_num = true;
            }
        }

        if has_num && !has_text && dot_count == 0 {
            Some(InputTypes::Integer(
                str.parse::<i32>().expect("parse fail to i32"),
            ))
        } else if has_num && !has_text && dot_count == 1 {
            Some(InputTypes::Float(
                str.parse::<f64>().expect("parse fail to f64"),
            ))
        } else {
            Some(InputTypes::Text(String::from(str)))
        }
    }
}

fn main() {
    let mut inputs: Vec<InputTypes> = Vec::new();

    println!("enter values 10 times..");
    let mut input = String::new();

    for i in 1..11 {
        print!("{}) ", i);
        std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut input)
            .expect("fail to read line");

        match InputTypes::get_for(input.trim()) {
            Some(input_type) => inputs.push(input_type),
            None => println!("fail to convert: {}", input.trim()),
        };

        input.clear();
    }

    let mut i_inputs = Vec::<i32>::new();
    let mut f_inputs = Vec::<f64>::new();
    let mut s_inputs = Vec::<String>::new();

    for v in inputs {
        match v {
            InputTypes::Integer(i) => i_inputs.push(i),
            InputTypes::Float(f) => f_inputs.push(f),
            InputTypes::Text(t) => s_inputs.push(t.clone()),
        }
    }

    i_inputs.sort();
    f_inputs.sort_by(|a, b| a.partial_cmp(b).expect("fail to cmp float"));
    s_inputs.sort();

    println!("sorted integers: {:?}", i_inputs);
    println!("sorted floats: {:?}", f_inputs);
    println!("sorted text: {:?}", s_inputs);
}


#[test]
fn test_get_for() {
    let i = InputTypes::get_for("2345");
    let f = InputTypes::get_for("12.54");
    let t = InputTypes::get_for("text");

    println!("{:?} {:?} {:?}", i, f, t);
}

#[test]
fn test_sort_string_vector() {
    let mut v = vec!["abc", "xyz", "bcd", "zxy", "cde"];
    v.sort();

    println!("{:?}", v);
}

#[test]
fn test_sort_float_vector() {
    let mut v = vec![10.4, 2.232, 1.2, 1.3, 8.4];
    v.sort_by(|a, b| a.partial_cmp(b).expect("fail to cmp"));

    println!("{:?}", v);
}

#[test]
fn test_parse_float() {
    println!("{}", "234.".parse::<f64>().unwrap());
}
