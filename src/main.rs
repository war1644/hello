#![allow(unused)]
//#![warn(unused_imports)]

mod http;
use crate::http::Http;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
//    Http::new();
    let (tx,rc) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("my"),
            String::from("is"),
            String::from("transmitter"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
            String::from("my2"),
            String::from("is2"),
            String::from("transmitter2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receive in rc {
        println!("msg: {}",receive);
    }
//    let msg = rc.recv().unwrap();

}


