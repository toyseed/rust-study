#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

impl Name {
    // method
    fn full_name(&self) -> String {
        // format!("{} {}", self.first, self.last)
        let mut full_name = String::new();
        full_name.push_str(&self.first);
        full_name.push_str(&" ");
        full_name.push_str(&self.last);

        full_name
    }

    // associated function
    fn say_hello() {
        println!("hello");
    }
}

// tuple structure
#[derive(Debug)]
struct Color(i32, i32, i32);

#[test]
fn test_print() {
    let name = Name {
        first: String::from("se"),
        last: String::from("song"),
    };

    println!("name is {:?}", name);
    println!("name is {:#?}", name);
    println!("full name is {}", name.full_name());
}

#[test]
fn test_tuple_structure() {
    let color = Color(10, 20, 30);
    println!("color is {:#?}", color);
}