/*
AStruct라는 객체를 사용하여 T로 i32와 f64, String을 사용하며,
키보드 인풋으로 key와 정수 또는  실수 또는 문자열을 입력받아서
열거형 Value에 저장하고, key에 따라 정렬하고, 출력하는
프로그램을 작성하세요.
그리고
AStruct, BStruct모두 ShowValue를 구현하세요,
show_value는 구조체의 value필드를 String에 찍어주는 기능으로 하세요.
*/

use std::fmt;
use std::fmt::{Display, Formatter};

trait ShowValue {
    fn show_value(&self) -> String;
}

enum Data {
    Integer(i32),
    Float(f64),
    String(String),
}

impl Data {
    fn from(str: &str) -> Data {
        if let Result::Ok(i) = str.parse::<i32>() {
            Data::Integer(i)
        } else if let Result::Ok(f) = str.parse::<f64>() {
            Data::Float(f)
        } else {
            Data::String(String::from(str))
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Data::Integer(i) => write!(f, "Integer({})", i),
            Data::Float(n) => write!(f, "Float({})", n),
            Data::String(s) => write!(f, "String({})", s),
        }
    }
}

struct Value<T>(T);

struct AStruct<'a, T> {
    key: &'a str,
    value: &'a Value<T>,
}

impl<T> AStruct<'_, T> {
    fn new<'a>(key: &'a str, value: &'a Value<T>) -> AStruct<'a, T> {
        AStruct { key, value }
    }
}

impl<T: Display> ShowValue for AStruct<'_, T> {
    fn show_value(&self) -> String {
        format!("{}: {}", self.key, self.value.0)
    }
}

struct BStruct {
    name: String,
    value: String,
}

impl ShowValue for BStruct {
    fn show_value(&self) -> String {
        self.value.clone()
    }
}

fn fetch_input() -> Option<(String, String)> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if &input == "\n" {
                return None;
            }

            let split: Vec<&str> = input.trim().split_whitespace().collect();

            if split.len() != 2 {
                return None;
            }

            Some((split[0].to_string(), split[1].to_string()))
        }
        Err(_) => None,
    }
}

fn main() {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<Value<Data>> = Vec::new();

    loop {
        match fetch_input() {
            Some((key, value)) => {
                keys.push(key);
                values.push(Value(Data::from(&value)));
            }
            None => break,
        }
    }

    let mut datas: Vec<AStruct<Data>> = Vec::new();

    for index in 0..keys.len() {
        datas.push(AStruct::new(&keys[index], &values[index]));
    }

    datas.sort_by_key(|data| data.key);

    for data in datas {
        println!("{}", data.show_value());
    }
}
