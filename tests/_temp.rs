
#[test]
fn test() {
    let a = 10;
    let b = 4;

    let c = a / b;
    let d = a % b;

    println!("{}, {}", c, d);
}

// struct Message(String);
//
// impl Copy for Message {
//
// }
//
// impl Clone for Message {
//     fn clone(&self) -> Self {
//         Message(self.0.clone())
//     }
// }

#[derive(Debug)]
struct CubeSat {
    id: i32,
    // message: Message,
}

impl Copy for CubeSat {

}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        *self
    }
}

#[test]
fn test_clone() {
    let cubesat = CubeSat {
        id: 0,
        // message: Message(String::from("hi")),
    };

    let new_cubesat = cubesat.clone();
    let copied_cubesat = cubesat;

    println!("cubsat: {:?}", cubesat);
    println!("new cubsat: {:?}", new_cubesat);
    println!("copied cubsat: {:?}", copied_cubesat);
}