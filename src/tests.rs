#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

// config for unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(larger.can_hold(&smaller));

    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    // #[should_panic]
    #[should_panic(expected = "OMG")]   // expect that the panic message contains expected string.
    fn it_should_panic() {
        panic!("OMG!");
    }

    #[test]
    // cannot use should_panic annotation to the function that returns Result type.
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {

    }
}