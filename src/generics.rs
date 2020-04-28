pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn mx(&self) -> f64 {
        &self.x * 10.0
    }
}

#[test]
fn test_main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // println!("p.x = {}", p.mx());

    let p = Point{ x: 10.5, y: 20.6 };
    println!("p.x = {}", p.mx());
}