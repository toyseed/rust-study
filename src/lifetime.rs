#[test]
fn test_lifetime() {
    {
        let x = 42;
        let x_ref = &x;

        println!("x_ref = {}", x_ref);
        println!("x = {}", x);
        println!("x_ref = {}", x_ref);
    }

    // println!("x = {}", x);
    //                    ^ not found in this scope
}

// #[test]
fn test_ref_lifetime() {
    // let x_ref = {
    //     let x = 42;
    //     &x
    //   //^^ borrowed value does not live long enough
    // };

    // println!("x_ref = {}", x_ref);
    // - `x` dropped here while still borrowed
}

#[test]
fn test_immutable_borrow() {
    let x = 42;
    let x_ref1 = &x;
    let x_ref2 = &x;
    let x_ref3 = &x;

    println!("{} {} {}", x_ref1, x_ref2, x_ref3);
}

#[test]
fn test_mutable_borrow() {
    let mut x = 42;

    let x_ref1 = &x;
    let x_ref2 = &mut x;

    // println!("x_ref1 = {}", x_ref1); // error
    println!("x_ref2 = {}", x_ref2);

    // let x_ref2 = &mut x;
    // let x_ref1 = &x; // immutable borrow occured after mutable borrow
}

struct NumRef<'a> {
    x: &'a i32,
}

fn as_num_ref(x: &i32) -> NumRef<'_> {
    NumRef { x: &x }
}

#[test]
fn test_elided() {
    let x: i32 = 99;
    let x_ref = as_num_ref(&x);

    println!("x_ref: {}", x_ref.x);

    let y = x;
    println!("x_ref: {}", x_ref.x);
}