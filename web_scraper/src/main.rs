use std::time::Duration;
use tokio::time::sleep;


// This is a helper function just like main it returns a Future that must be .await ed 
// Helper function that tries to fetch a URL with a retry loop
async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, String> {
    let mut attempts = 0;

    while attempts < max_retries {
        println!("Attempt {}/{} for {}", attempts + 1, max_retries, url);
    

        // Try the network request
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {

                // EXtract the HTML body as a string
                // Earlier we just printed response.status(). Now we actually want the website data The .text() method download the HTML body. Notice it has a .await on it
                // That's because downloading a large 10 MB HTML file from a scoket takes time so that operation is also asynchronous

                let html = response.text().await.unwrap();

                // If the request succeds we immediately exit the while loop and returns the HTML string
                return Ok(html);
            } else {
                println!("Server returned error: {}", response.status());
            }
        },
        Err(e) => {
            println!("Network Error: {}", e);
        }
    }

    attempts += 1;

    if attempts < max_retries {
        println!("Sleeping for 3 seconds before next attenpt ..\n");

        // We use tokio::time::sleep . If we used std::thread::sleep here
        // we would freeze the entire tokio executor
        // The kitchen timer. We wait 3 seconds without blocking the OS thread 
        sleep(Duration::from_secs(3)).await;
        }
    }

    Err(format!("Failed to fetch {} after {} attempts.", url, max_retries))
}

#[tokio::main]
async fn main() {
    // Intentionally test a fake url that doesn't exist to trigger the retries
    let bad_url = "https://this-website-definitely-does-not-exist-12345.com";

    match fetch_with_retry(bad_url, 3).await {
        Ok(html) => println!("Success! Downloaded {} bytes of HTML.", html.len()),
        Err(e) => println!("Final Failure: {}", e),
    }
}