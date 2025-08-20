// Concurrency: The ability of different parts of a program to execute independently.
// Parallelism: The ability of different parts of a program to execute at the same time.

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    // SECTION: CREATING A NEW THREAD WITH `spawn`
    // The `thread::spawn` function takes a closure, which is the code to be run in the new thread.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread will continue its execution.
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // The `join` method on the handle waits for the spawned thread to finish.
    handle.join().unwrap();


    // SECTION: USING `move` CLOSURES WITH THREADS
    // The `move` keyword is often used with closures passed to `thread::spawn`
    // to force the closure to take ownership of the values it uses.
    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();


    // SECTION: MESSAGE PASSING WITH CHANNELS
    // Channels are a way for threads to communicate with each other.
    // `mpsc` stands for multiple producer, single consumer.
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone(); // Clone the transmitter to have multiple producers

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // The `recv` method on the receiver will block the main thread's execution
    // and wait until a value is sent down the channel.
    for received in rx {
        println!("Got: {}", received);
    }


    // SECTION: SHARED-STATE CONCURRENCY WITH MUTEX
    // A Mutex (mutual exclusion) is a way to allow only one thread to access some data at any given time.
    // To access the data in the mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock.

    // `Arc<T>` is an "Atomically Reference Counted" type, which is a thread-safe version of `Rc<T>`.
    // We use it to share ownership of the mutex across multiple threads.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result of mutex counter: {}", *counter.lock().unwrap());
}
