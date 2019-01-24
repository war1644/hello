use std::io::prelude::{BufRead, Read, Seek, Write};
use std::fs;
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::time::Duration;

// thread pool
struct ThreadPool;
impl ThreadPool {
    fn new(size: u32) -> ThreadPool { ThreadPool }
    fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {}
}

pub struct Http;
impl Http
{
    pub fn new(){
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        let pool = ThreadPool::new(4);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                Self::handle_connection(stream);
            });
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
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