/*
AStruct라는 객체를 사용하여 T로 i32와 f64, String을 사용하며,
키보드 인풋으로 key와 정수 또는  실수 또는 문자열을 입력받아서
열거형 Value에 저장하고, key에 따라 정렬하고, 출력하는
프로그램을 작성하세요.
그리고
AStruct, BStruct모두 ShowValue를 구현하세요,
show_value는 구조체의 value필드를 String에 찍어주는 기능으로 하세요.

**접근법이 잘못되어 다시 구현함 -> ex3_save_sort_show.rs**
*/

use std::cmp::Ordering;
use std::fmt::Debug;

trait ShowValue {
    fn show_value(&self) -> String;
}

enum Data {
    Integer(i32),
    Float(f64),
    String(String),
}


#[derive(Debug)]
enum Value<T> {
    V(T),
}

impl<T: ToString> ToString for Value<T> {
    fn to_string(&self) -> String {
        match self {
            Value::V(i) => format!("{}", i.to_string()),
            _ => panic!("panic!!!!"),
        }
    }
}

#[derive(Debug)]
struct AStruct<'a, T> {
    key: &'a str,
    value: &'a Value<T>,
}

impl<T: ToString> ShowValue for AStruct<'_, T> {
    fn show_value(&self) -> String {
        Value::to_string(self.value)
    }
}

impl<T> Eq for AStruct<'_, T> {}

impl<T> Ord for AStruct<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.key.partial_cmp(other.key) {
            Some(order) => order,
            None => panic!("ordering error"),
        }
    }
}

impl<T> PartialEq for AStruct<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

impl<T> PartialOrd for AStruct<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(other.key)
    }
}

struct BStruct {
    name: String,
    value: String,
}

impl ShowValue for BStruct {
    fn show_value(&self) -> String {
        String::from(&self.value)
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astruct_sort() {
        let i_value = Value::V(1);
        let f_value = Value::V(20.2);
        let s_value = Value::V("hello");
        let item_1 = AStruct {
            key: "z",
            value: &i_value,
        };
        let item_2 = AStruct {
            key: "a",
            value: &f_value,
        };
        let item_3 = AStruct {
            key: "g",
            value: &s_value,
        };

        // let mut v : Vec<(&str, &Value<_>)> = vec![
        //     (item_1.key, item_1.value), (item_2.key, item_2.value)
        // ];
        //
        // v.sort();
        //
        // println!("{:?}", v);
    }
    #[test]
    fn test_astruct() {
        let key = String::from("f_key");
        let value = Value::V("hello");

        let a = AStruct {
            key: &key,
            value: &value,
        };

        println!("{}: {}", a.key, a.show_value());
    }

    #[test]
    fn test_value_enum() {
        let v = Value::V(222);
        println!("{}", v.to_string());
    }

    #[test]
    fn test_showvalue_of_bstruct() {
        let bstruct = BStruct {
            name: String::from("name"),
            value: String::from("value"),
        };
        println!("{}", bstruct.show_value());
    }
}
