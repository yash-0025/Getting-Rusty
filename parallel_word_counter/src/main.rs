use std::thread;
use std::time::Duration;


fn main() {
    // Give the main thread some data
    let my_data = String::from("Secret Recipe");

    // Spawn a new OS thread The line cook
    // thread::spawn This function asks the Operating system to create a brand new physical hardware thread

    // move || {...}  - This is the closure (the recipe ) we give the thread. We must use the move keyword because the closure is capturing my_data from the main function. If we didn't use move the thread would only borrow my_data and rust would panic because the main thread might finish and destroy my_data before the new thread finishes

    // let handle = spawn returns a JoinHandle. This is like a pager we get at a restaurant . It allows us to check on the thread or wait for it.

    let handle = thread::spawn(move || {
        println!("Line Cook : Started working on {}", my_data);
        thread::sleep(Duration::from_secs(2));
        println!("Line cook: Finished");

        // Closure returns a value back to the main
        return 42;
    });
    // Main thread keeps workign simultaneuously
    println!("Main Chef : I am boiling water while the cook works!");
    thread::sleep(Duration::from_secs(1));
    println!("Main Chef: Water is boiling");

    // Wait for the line cook to finish
    // handle.join() - this blocks the main thread . the main thread will literally pause execution on this exact line until the spawned thread finishes and returns it  value (42)

    // .unwrap() - If the spawned thread crashes (panics), .join() will return an Err. We use .unwrap() to say if the line cook crashed I want to crash too.
    let result = handle.join().unwrap();
    println!("Main chef: The cook returned the number: {}", result);
}
