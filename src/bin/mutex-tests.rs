use std::sync::{Mutex, Arc};
use std::thread;

fn test_mutex() {
    let num = Mutex::new(5);

    {
        let mut num = num.lock().unwrap();
        *num = 10;
    }

    println!("{:?}", num);
}

fn test_mutex_in_multi_thread() {
    let mut count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            *num  += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("test_mutex_in_multi_thread result: {}", *count.lock().unwrap());
}
fn main() {
    test_mutex();
}
