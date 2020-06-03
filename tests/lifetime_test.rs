
struct LifeTime<'a> {
    s: &'a str,
}
impl<'a> LifeTime<'a> {
    fn longest(&mut self, x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            self.s = x;
            x
        } else {
            self.s = y;
            y
        }
    }

    fn print_something(&self) {
        println!("haha");
    }
}

// struct LifeTime2 {
//
// }
//
// impl LifeTime2 {
//     fn longest(&self, x: &str, y: &str) -> &str {
//         if x.len() > y.len() {
//             x
//         } else {
//             y
//         }
//     }
// }
#[test]
fn test_lifetime() {
    let mut l = LifeTime {s:""};
    let x = "hello";

    {
        let y = String::from("world");
        let r = l.longest(x, y.as_str());

        println!("y: {}", r);
    }

    // l.print_something();     // compile error
}

fn test_generic<T>(a: &T) {

}