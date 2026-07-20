
// #[tokio::main] - This is a procedural macro [an attribute macro] It intercepts the main function before it compiles and injects all the boilerplate code needed to start a multithreaded Green Thread Pool [The tokio runtime]
// Without this we cannot use the .await keyword
#[tokio::main]
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