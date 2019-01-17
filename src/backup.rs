use std::fs::File;
use std::io::Read;
use std::io;


/// 读取file
fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let rand_number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("input your number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue

        };//.expect("Please type a number!");

        println!("You input : {},{}", guess, rand_number);
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;

    fn main() -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for i in 0..101 {
            handle.write_fmt(format_args!("\rprogress: {}%", i))?;
            thread::sleep(Duration::from_millis(100));
            handle.flush()?;
        }
        println!("\n");
        Ok(())
    }
}









