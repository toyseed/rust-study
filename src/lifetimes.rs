// error
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn print_longest(x: &str, y: &str) {
    println!("{}", x);
}

#[test]
fn test() {
    let x = String::from("abcd");
    let result;
    // let result2;

    {

        // let y = String::from("xyzxyz");
        // result = longest(&x, &y);
        // let y = "xyzxyz";
        // result = longest(&x, y);

        // 테스트 하려고 했던건 result 가 x 의 reference 지만 lifetime 으로 인해
        // result 가 vailidate 하지 못하게 되는 것
        // but 컴파일러가 longest 의 리턴 타입의 lifetime 과 param 의 lifetime 이 달라 컴파일 오류났음.
        let y = String::from("hi");
        result = longest(&x, &y);
        // result2 = longest(&y, &x);
        print_longest(&x, &y);

        let result_2 = longest_2(&x, &y);
    }

        // println!("{}", result);
    // println!("{}", result2);
}

#[derive(Debug)]
struct MutableRef<'a> {
    part: &'a str,
}

#[test]
fn test_struct_lifetime() {
    let s = String::from("outer");
    let mut mr = MutableRef { part: &s };
    {
        let i = String::from("inner");
        mr.part = &i;
    }
    // println!("{:?}", mr);
}
