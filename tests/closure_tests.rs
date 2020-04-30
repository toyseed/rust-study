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

fn for_each_planet_with_static<F>(f: F)
    where
        F: Fn(&'static str) + 'static,  // need function has static lifetime
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

#[test]
fn test_static_closure() {
    let greeting = String::from("Good to see you");
    // this 'closure' has same 'lifetime' with 'greeting'. so codes below cannot be compiled.
    // for_each_planet_with_static(|planet| println!("{}, {}", greeting, planet));
    for_each_planet_with_static(move |planet| println!("{}, {}", greeting, planet));
}

fn make_tester<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| {
        challenge == answer
    }
}

#[test]
fn test_return_closure() {
    let test = make_tester("hunter2");
    println!("{}", test("******"));
    println!("{}", test("hunter2"));
}