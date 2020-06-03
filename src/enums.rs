#[cfg(test)]
mod tests {
    enum TestEnum {
        Struct {
            x: u8, y: u8,
        },
        Tuple (u8, u8),
    }

    #[test]
    fn test_test_enum() {
        let s = TestEnum::Struct {x: 8, y: 10};
        let t = TestEnum::Tuple(11, 12);

        // println!("s.x: {}", s.x);
        // println!("t.0: {}", t.0);

        if let TestEnum::Struct{x, y} = &s {
            println!("s.x: {}", x);
        }

        if let TestEnum::Tuple(a, b) = &t {
            println!("t.0: {}", a);
        }
    }

    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl IpAddr {
        fn print(&self) {
            println!("IpAddr {:?}", self);
            match self {
                IpAddr::V4(v1, v2, v3, v4) => {
                    println!("first value: {}", v1);
                }
                _ => println!("hm..."),
            }
        }
    }

    #[test]
    fn test_enum() {
        let ip_addr = IpAddr::V4(1, 2, 3, 4);
        ip_addr.print();

        let some_number: Option<u8> = Some(10);
    }

    fn plug_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    #[test]
    fn test_option() {
        let five = Some(5);
        let six = plug_one(five);
        let none = plug_one(None);
    }

    #[test]
    fn test_match_exhaustive() {
        let some_num = 0u8;

        match some_num {
            1 => println!("one"),
            _ => (),
            // `_` : extra case
            // (): unit value
        }
    }

    #[test]
    fn test_if_let() {
        let some_u8_value = Some(4u8);

        match some_u8_value {
            Some(3) => println!("three from match"),
            _ => (),
        }

        // it works, but it cannot get actual value of option
        if Some(4) == some_u8_value {
            println!("test");
        }

        // syntactic sugar for match
        if let Some(i) = some_u8_value {
            println!("three from if let: {}", i);
        } else if let Some(4) = some_u8_value {
            println!("four from if let");
        }
    }

    // #[test]
    // fn test_none() {
    //     let num = Some(3);
    //     let result = match num {
    //         Some(0) => 0,
    //         None => None,
    //         _ => (),
    //     };
    //
    //     result.add(10);
    // }
}
