#[test]
fn test_emoji_string_slice() {
    let emoji = "👩‍❤️‍👨";
    println!("{}", &emoji);
    // println!("{}", &emoji[..1]); // panic!
    println!("{}", &emoji[..4]);
}

#[test]
fn test_return_string_slice() {
    fn give_me_slice(s: &str) -> &str{
        &s[..]
    }

    let mut source = String::from("hello string");
    let slice = give_me_slice(&source);

    source.clear();

    // if immutably borrowed variable is not used, it's not problem to use the owner mutably.
    // println!("{}", slice);

    let source = "hello slice";
    let slice = give_me_slice(source);
}