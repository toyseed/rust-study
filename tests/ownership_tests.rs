#[derive(Debug)]
struct Number {
    num: i32,
}

fn neg_print(num: &mut Number) -> &Number {
    num.num = -1 * num.num;
    num
}

#[test]
fn test_owner() {
    let mut num = Number { num: 10 };

    println!("{:?}", num);
    println!("{:?}", neg_print(&mut num));
    println!("{:?}", num);

    let imu_num = &num;
    println!("{:?}", imu_num);
    let mu_num = &mut num;
    println!("{:?}", mu_num);
    // println!("{:?}", imu_num);

    use std::any::type_name;

    println!("{:?}", type_name::<i32>());
}

#[derive(Debug)]
struct Person {
    name: &'static str,
}

fn give_me_a_person(p: Person) {
    println!("got a person");
}

fn give_me_a_person_in_the_box(p: &Box<Person>) {
    println!("got a person in the box");
    // let person = *p;
}

#[test]
fn test_static() {
    let p1 = Person { name: "hello rust" };
    let p1_1 = p1;

    let p1 = Person { name: "hello rust" };
    // let p1_1 = *&p1;

    give_me_a_person(p1);

    let p2 = Box::new(Person { name: "hello rust" });
    // let p2_2 = p2;
    // let p2_1 = *&p2;
}

struct Data {
    value: i32,
}

impl Data {
    fn take_ownership(mut self) {
        println!("execute Data::take_ownership");
    }
}

struct DataOwner {
    data: Data,
}

impl DataOwner {
    fn borrow_self(&self) {
        println!("DataOwner::borrow_self executed");
    }
}

struct OptionDataOwner {
    data: Option<Data>,
}

impl OptionDataOwner {
    fn borrow_self(&self) {
        println!("OptionDataOwner::borrow_self executed");
    }
}

#[test]
fn test_data_owner() {
    let data_owner = DataOwner {
        data: Data { value: 10 },
    };

    data_owner.data.take_ownership();
    // data_owner.borrow_self();

    let mut option_data_owner = OptionDataOwner {
        data: Some(Data { value: 11 }),
    };

    if let Some(data) = option_data_owner.data.take() {
        data.take_ownership();
    }

    option_data_owner.borrow_self();

    if let None = option_data_owner.data {
        println!("option_data_owner.data is none");
    }
}
