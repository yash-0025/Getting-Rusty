use std::sync::Arc;
use std::time::{Duration, Instant}; //  Added for measuring latency and defining the timeout
use tokio::sync::Semaphore;
use tokio::time::timeout; // For the timeout 



// #[tokio::main] - This is a procedural macro [an attribute macro] It intercepts the main function before it compiles and injects all the boilerplate code needed to start a multithreaded Green Thread Pool [The tokio runtime]
// Without this we cannot use the .await keyword
// #[tokio::main]
// async fn main() - We added the async keyword to main function. This allows us to use .await inside the function body. The macro above makes this valid
/* async fn main() {
    println!("Tokio runtime has started~");

    let url = "https://httpbin.org/status/200";
    println!("Fetching {}...", url);

    // reqwest::get(url) - this starts an asynchronous HTTP GET request. Crucially it returns a Future not the actual response data. 
    // .await - This is the magic keyword. WE attach this to the end of the Future . This explicitly hands the future to the Tokio runtime and says pause this function until the network request finishes 
    // .unwrap() - Because network request can fail eg- no WIFI , invalid URL, server crash. .await actually resolves into a Result<Response,Error>. We use .unwrap() here to say if it fails just crash the program (we will handle errors gracefully later)


    let response = reqwest::get(url).await.unwrap();

    // response.status() - Extracts the HTTP code like (200, 404, 500) from the returned reqwest::Response object so we can print it
    println!("Status: {}", response.status());

}
 */

/* 
 async fn main() {
    

    let urls = vec![
        "https://google.com",
        "https://patelyash.in",
        "https://github.com",
        "https://httpbin.org/status/200",
        "https://httpbin.org/status/404",
    ];

    // WE need an empty vector to store the tickets JoinHandles that Tokio gives us

    let mut handles = vec![];

    for url in urls {

        // tokio::spawn is the async equivalent of std::thread::spawn
        // async move - The move keyword forces the closure to take ownership of the url variable. if we didn't do this the background task would just have a reference to url, which could be dangerous if the main function finished before the task did causing a dangling pointer 
        let handle = tokio::spawn(async move {
            println!("Fetching {}...", url);
            let response = reqwest::get(url).await.unwrap();
            println!("{} returned {}", url, response.status());
        });

        // We store the ticket

        handles.push(handle);
    }

    for handle in handles {

        // the first .await inside the taask was waiting for the network. This .await is waiting for the task itself to finish.
        // the .unwrap() handles the rare case where the Tokio task panicked internally.
        handle.await.unwrap();
    }
 }
  */

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://google.com",
        "https://patelyash.in",
        "https://github.com",
        "https://httpbin.org/status/200",
        "https://httpbin.org/status/404", // This url artificially delays for 5 seconds to test our timeout 
        "https://httpbin.org/delays/5", // this url artificially delays for 5 seconds to test our timeout 
    ];

    // Creates a bouncer with exactly 2 VIP wristbands
    // We wrap it in ARc so multiple tasks can share ownership of the bouncer

    // Arc::new(Semaphore::new(2)) - We use Arc Atomic Reference Counted pointer because the Semaphore needs to be shared across many concurrent background tasks
    let semaphore = Arc::new(Semaphore::new(2));

    let mut handles = vec![];


    println!("{:<35} | {:<20} | {}", "URL", "STATUS",  "LATENCY (ms)");
    println!("{:-<35}-+-{:-<20}-+-{:-<10}","", "", ""); // Prints a dividing line

    for url in urls {
        // let sem_clone = Arc::clone(&semaphore) - we creates a new pointer to the exact same Semaphore . We do this inside the loop so that the async move closure takes ownership of the clone not the original .
        let sem_clone = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {

            // let _permit = sem_clone.acquire().await.unwrap() - This is where the task stops and waits in line. The _permit variable holds the wristband . We use the _ prefix because we never actually use the permit variable in our code its mere existence in memory is what keeps the slot reserved

            // ASk the bouncer for a wristband. If none are available sleep here
            let _permit = sem_clone.acquire().await.unwrap();

            // Start a Stopwatch
            // Grabs the current CPU clock time
            let start_time = Instant::now();

            // We wrap our network request in tokio::time::timeout
            // It takes a Duration (2 seconds) and a Future (the request get)

            // We literally put the reqwest Future inside the timeout Future. The timeout Future races a 2 second timer against the network request. whichever finishes first wins
            let result = timeout(Duration::from_secs(2), reqwest::get(url)).await;

            // STop the stopwatch
            // Calculates the difference between now and start_time , formatting it as milliseconds
            let latency = start_time.elapsed().as_millis();

            // Match on the outer Result (the Timeour)
            // Because we have a Future inside a Future we have nested Result The outer REsult tells us if the timeout fired. 
            // The inner Result tells us if the network request failed eg DNS error
            match result {
                Ok(network_result) => {
                    // The request finished before 2 seconds . Now check if the network request itself succeded
                    match network_result {
                        Ok(response) => println!("{:<35} | {:<20} | {}ms", url, response.status(), latency),
                        Err(e) => println!("{:<35} | {:<20} | {}ms", url, "NETWORK ERROR", latency),
                    }
                },
                Err(_) => {
                    println!("{:<35} | {:<20} | {}ms", url , "TIMEOUT", latency);
                }
            }

            // let response = reqwest::get(url).await.unwrap();


            // The {:<35} syntac tells Rust to left-align the string and pad it with spaces until it is exactly 35 characters wide. This creates a perfect vertical table column
            // println!("{:<35} | {}", url, response.status());


            // When the task ends here , _permit goes out of scope and is dropped
            // this automatically returns the wristband to the bouncer
        });

        handles.push(handle);

    }

    for handle in handles {
        handle.await.unwrap();
    }
}