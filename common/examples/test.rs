use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(val) => println!("Recieved: {}", val),
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    });
    thread::sleep(Duration::from_millis(1000));
    tx.send(1).unwrap();
    thread::sleep(Duration::from_millis(1000));
    panic!();
}
