use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..5 {
        let data_ref = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data.push(i);
            println!("Thread {} inseriu o valor {}", i, i);        
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Vetor final: {:?}", shared_data.lock().unwrap());
}
