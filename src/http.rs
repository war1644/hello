use std::io::prelude::{BufRead, Read, Seek, Write};
use std::fs;
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::time::Duration;
use std::sync::{mpsc,Arc,Mutex};

type Job = Box<FnOnce() + Send + 'static>;
// save id and join handle
struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
}
impl Worker {
    fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} start executing.", id);
                (*job)();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}

// thread pool
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender
        }
    }
    fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

//    pub fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
//        where
//            F: FnOnce() -> T + Send + 'static,
//            T: Send + 'static
}

pub struct Http;
impl Http
{
    pub fn new(){
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        let pool = ThreadPool::new(4);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || {
                Self::handle_connection(stream);
            });
        }
    }

    pub fn handle_connection(mut stream: TcpStream,) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
        };
        let content = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}",status_line,content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}