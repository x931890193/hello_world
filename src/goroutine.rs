use std::thread;
use std::time::Duration;
use std::sync:: {Arc, Mutex};
use std::sync::mpsc;
use std::cell::Ref;


fn main() {
    let handle = thread::spawn(|| -> u32{
        println!("hello from thread");
        1
    });
    println!("hello {}", handle.join().unwrap());
    bad_demo();

}

fn bad_demo() {
    let data = Arc::new(Mutex::new(vec![0u32,1,2,3,4,5]));
    for i in 0..6 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
            println!("{} random order!", data[i]);
        });
    }
    thread::sleep(Duration::from_millis(50));
}

// sync thread use channel
fn not_good_demo() {
    let data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            tx.send(Ref::new(data));
        });
    }
    for _ in 0..10 {
        match rx.recv() {
            Ok(x) => println!("{}", x),
            _ => (),
        }
    }
}