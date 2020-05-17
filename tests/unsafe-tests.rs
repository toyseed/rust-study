#[test]
fn test_unsafe_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {}
fn inner_dangerous() {
    unsafe {
        println!("inner unsafe");
    }
}
#[test]
fn test_unsafe_function_call() {

    unsafe {
        dangerous();
    }

    inner_dangerous();
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static GLOBAL_VARIABLE: &str = "Hello, world!";

unsafe trait Foo {

}

unsafe impl Foo for i32 {

}
