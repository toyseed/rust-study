use std::thread;
use std::thread::JoinHandle;

fn main() {
    let c = || println!("this is closure");

    // take_closure(c);
    take_closure_and_send(c);
    // let handle = take_closure_in_thread(c);
    // handle.join();
}

fn take_closure_and_send<F>(f: F)
where
    F: Fn() + Send,
{
    take_closure(f);
}
fn take_closure<F>(f: F)
where
    F: Fn(),
{
    f()
}

fn take_closure_in_thread<F>(f: F) -> JoinHandle<()>
where
    F: Fn() + Send + 'static,
{
    // let f = Box::new(f);
    thread::spawn(f)
}
