use std::rc::Rc;
use std::borrow::Borrow;

struct Temp(usize);

impl Temp {
    fn new(n: usize) -> Temp {
        Temp(n)
    }

    fn t_m(&self) {
        print!("my number is {}", self.0);
    }

    pub fn r_m(&self) -> (Rc<Temp>, fn(&Temp)) {
        let _temp = Temp::new(4);
        let _temp = Rc::new(_temp);

        (Rc::clone(&_temp), Temp::t_m)
    }
}

#[test]
fn test_pass() {
    let temp = Temp::new(10);
    let (_temp, _t_m) = temp.r_m();

    _t_m(_temp.borrow());
}
