use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use std::sync::{mpsc, Arc, Mutex};
use std::{fs, str, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type SharedJobReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

enum Message {
    Job(Job),
    Stop,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        // this closure needs lifetime annotation.
        // because it is going to be executed on another thread.
        // if it doesn't have lifetime annotation, rust compiler can not confirm validity of the closure.
        F: FnOnce() + Send + 'static,
    {
        // closure is not a type. so it can't be sent as a type object.
        // it needs to be sent as a trait object.
        // with `Box` you can make a trait object.
        let job = Box::new(f);
        self.sender.send(Message::Job(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(Message::Stop).unwrap();
        }

        println!("Shutting down all workers");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: SharedJobReceiver) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                match message {
                    Message::Job(job) => {
                        job();
                    }
                    Message::Stop => {
                        break;
                    }
                }
            }
            println!("worker {} is terminated", id);
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// https://users.rust-lang.org/t/rust-web-server-not-working-with-requests-from-chrome/34835/8
fn handle_connection(mut stream: TcpStream) {
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    let mut buffer = String::new();
    // stream.read_to_string(&mut buffer).unwrap();

    let mut buffer = [0; 64];
    let mut result = String::new();

    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }

        println!("!!!!! {}, {}", size, str::from_utf8(&buffer).unwrap());
        result.push_str(str::from_utf8(&buffer).unwrap());
        // println!("inner result: {}", result);
    }

    println!("while breaked");
    // println!("request: {}", result);

    let get = b"GET / HTTP/1.1\r\n";
    let get = "GET / HTTP/1.1\r\n";

    // let (status_line, filename) = if buffer.starts_with(get) {
    let (status_line, filename) = if result.starts_with(get) {
        println!("with 200");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        println!("with 404");
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    println!("read file");
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    println!("send back to client");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
