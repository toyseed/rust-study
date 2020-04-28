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

#[test]
fn test_static() {
    let p1 = Person { name: "hello rust" };

    // let p2 = Person { name: &String::from("hello heap") };
}