#[test]
fn test_mutability_of_nested_struct() {
    struct Inner {
        value: u8,
        str_value: &'static str,
        string_value: String,
    }

    struct Outer {
        value: Inner,
    }

    let mut outer = Outer {
        value: Inner {
            value: 10,
            str_value: "hello_inner",
            string_value: String::from("hello string"),
        },
    };

    println!("{}", outer.value.value);
    outer.value.value = 0;
    println!("{}", outer.value.value);

    let copied_outer = Outer {
        ..outer     // move occurred
    };

    println!("{}", copied_outer.value.value);
    println!("{}", copied_outer.value.str_value);
    println!("{}", copied_outer.value.string_value);
}

#[test]
fn test_tuple_struct() {
    struct TupleStruct(i32, String);

    let tuple_struct = TupleStruct(100, String::from("hello tuple struct"));
    println!("{}", tuple_struct.0);
    println!("{}", tuple_struct.1);
}