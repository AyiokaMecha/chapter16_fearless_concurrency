 use std::thread;
 use std::time::Duration;
 use std::sync::{Arc, mpsc};

 fn main() {
    println!("Hello, world!");
    let v = vec![1, 3, 2, 5];
    let handle =  thread::spawn(move || {
          println!("Hellow from thread 1");
          thread::sleep(Duration::from_millis(200));
        println!("{:?}", v);
      });

     for i in 1..5 {
         println!("hi number {i} here!");
         thread::sleep(Duration::from_millis(250));
     }

     //join blocks the thread until the entire minithread has finished its work
     handle.join().unwrap_or_else(|_| {
         dbg!("Red flag red flag");
     });

     let (tx, rx) = mpsc::channel::<String>();

}


 fn shared_state() {
     use std::sync::Mutex;

     let m = Mutex::new(5);

     {
         let mut num = m.lock().unwrap();
         *num = 6;
     }

     println!("m = {:?}", m);

     let counter = Arc::new(Mutex::new(0));
     let mut handles = Vec::new();

     for _ in 1..10 {
         let counter = Arc::clone(&counter);
         let handle = thread::spawn(move || {
             let mut num = counter.lock().unwrap();
             *num += 1;
         });

         handles.push(handle);
     }
 }