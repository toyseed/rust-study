use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn change(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

#[test]
fn test_mut() {
    let mut p = Point { x: 0, y: 0 };

    println!("{:?}", p);

    p.change(5, 6);

    println!("{:?}", p);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where
    T: Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[test]
fn test_refcell() {
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            {
                let mut sent_message = self.sent_messages.try_borrow_mut().unwrap();
                sent_message.push(String::from(msg));
            }
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    let mock_messenger = MockMessenger::new();
    mock_messenger.send("hello refcell");
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
}

// #[test]
// fn test_temp() {
//     let point = Point { x: 1, y: 2 };
//     let rp = &point;
//
//     println!("{}, {}, {}, {}", &(point.x), &point.x, (&point).x, point.x);
//     println!("{}, {}", &(point.x) == point.x, (&point).x == *&point.x);
// }

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

#[test]
fn test_groundstation() {
    let base = Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));
    println!("base: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
