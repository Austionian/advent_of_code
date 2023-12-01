use anyhow::Result;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let weak_node = RefCell::new(Weak::new());

    let std_rc = Rc::new(0);

    *weak_node.borrow_mut() = Rc::downgrade(&std_rc);

    println!("{}", Rc::strong_count(&std_rc));
    println!("{}", Rc::weak_count(&std_rc));

    let v = vec![1, 2, 3];

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    let m = Mutex::new(2);

    let handle = thread::spawn(move || {
        println!("{}", v[1]);
        let val = String::from("hello from the thread!");
        let val2 = String::from("another message from the thread!");
        let vals = vec![val, val2];
        // for i in 0..10 {
        //     println!("thread: {i}");
        // }
        for value in vals {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    {
        let mut num = m.lock().unwrap();
        *num = 9;
    }

    let second_handle = thread::spawn(move || {
        let val = String::from("hello from second thread!");
        tx2.send(val).unwrap();
    });

    for i in 0..10 {
        println!("main: {i}");
        thread::sleep(Duration::from_millis(2));
    }

    for message in rx {
        println!("Got: {message}");
    }

    handle.join().unwrap();
    second_handle.join().unwrap();

    println!("m = {:?}", m);

    let mut handles = Vec::new();
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());

    Ok(())
}
