#[test]
fn test_pattern() {
    let (a, b, _, _) = (1, 2, 3, 4);
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    let (a, b, ..) = (4, 3, 2, 1);
    assert_eq!(a, 4);
    assert_eq!(b, 3);
}

struct Value {
    value: String,
}

// fn function_takes_tuple_ref(&(v1, v2): &(Value, Value)) {
//
// }

#[test]
fn test_function() {
    let v1 = Value {
        value: String::from("hi"),
    };

    let v2 = Value {
        value: String::from("hi"),
    };

    let tv = (v1, v2);
    // function_takes_tuple_ref(&tv);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[test]
fn test_struct_destructuring() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    println!("{:?}", p);
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[test]
fn test_enum_destructuring() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {}
        Message::Move { x, y } => {}
        Message::Write(text) => {}
        Message::ChangeColor(r, g, b) => {}
    }
}

#[test]
fn test_nested_enum_destructuring() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        ChangeColor(Color),
    }

    let message = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {}
        Message::ChangeColor(Color::Hsv(h, s, v)) => {}
        _ => (),
    }
}

#[test]
fn test_match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => {}
        Some(x) => {}
        None => {}
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

#[test]
fn test_at_binding() {
    // enum Message {
    //     Hello { id: i32 },
    // }
    //
    // let msg = Message::Hello { id: 5 };
    //
    // match msg {
    //     Message::Hello {
    //         id: id_variable @ 3..=7,
    //     } => println!("Found an id in rage: {}", id_variable),
    //     Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
    //     Message::Hello { id } => println!("Found some other id: {}", id),
    // }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
