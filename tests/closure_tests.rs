use std::collections::HashMap;

fn for_each_planet<F>(f: F)
where
    F: Fn(&'static str),
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

#[test]
fn test_closure() {
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));
}

fn for_each_planet_with_static<F>(f: F) -> Box<dyn Fn(&str)>
where
    F: Fn(&'static str) + 'static, // need function has static lifetime
{
    f("Earth");
    f("Mars");
    f("Jupiter");

    Box::new(|s: &str| println!("closure"))
}

#[test]
fn test_static_closure() {
    let greeting = String::from("Good to see you");
    // this 'closure' has same 'lifetime' with 'greeting'. so codes below cannot be compiled.
    // for_each_planet_with_static(|planet| println!("{}, {}", greeting, planet));
    for_each_planet_with_static(move |planet| println!("{}, {}", greeting, planet));
}

fn make_tester<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| challenge == answer
}

#[test]
fn test_return_closure() {
    let test = make_tester("hunter2");
    println!("{}", test("******"));
    println!("{}", test("hunter2"));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
            values: HashMap::new(),
        }
    }

    fn values(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                println!("from None");
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                println!("from None");
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn test_chacher() {
    let mut cacher = Cacher::new(|x| x + 1);

    let result = cacher.values(32);
    println!("{:?}", result);
    let result = cacher.values(32);
    println!("{:?}", result);
    let result = cacher.values(33);
    println!("{:?}", result);
    let result = cacher.values(33);
    println!("{:?}", result);
}

#[test]
fn test_capture() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn test_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("{:?}", x);

    let y = vec![1, 2, 3];
    let z = vec![1, 2, 3];

    assert!(equal_to_x(y));
    assert!(equal_to_x(z));
}

#[test]
fn test_fn() {
    let x = 4;

    let equal_to_x = |y: i32| y == x;

    println!("{}", x);

}

