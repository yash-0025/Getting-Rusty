use std::time::Duration;
use tokio::time::sleep;
use scraper::{Html, Selector}; // Import our HTML parser



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
    // let bad_url = "https://this-website-definitely-does-not-exist-12345.com";

    let url = "https://www.rust-lang.org/";

    // match fetch_with_retry(bad_url, 3).await {
    //     Ok(html) => println!("Success! Downloaded {} bytes of HTML.", html.len()),
    //     Err(e) => println!("Final Failure: {}", e),
    // }

    match fetch_with_retry(url, 3).await {
        Ok(html) => {
            // We hand the raw string to the librarian. it builds the massive document tree in memory

            // It takes a reference to the giant string and does the heavy CPU work of turning it into a queryable datastructure
            let document = Html::parse_document(&html);


            // We write our sticky note. We want elements that match the title tag
            // .unwrap() is used because parsing a selector can fail if we write invalid CSS syntax
            // Compile the CSS rule. If we wanted to find links , we put "a" . if we wanted elements with a specific alss we put ".class-name".
            let title_selector = Selector::parse("title").unwrap();

            // We ask the lirarian to find everything matching our sticky not
            // document.select() returns an Iterator (there could be multiple title tags)
            // .next() grabs the very first one it finds
            // document.select(&title_selector) = Searches the DOM . Returns an iterator, because CSS selectors usually match multiple things on a page
            // .next() - Grabs the very first match if it exists it unwraps it into title_element
            if let Some(title_element) = document.select(&title_selector).next() {
                // .inner_html() strips away the <title> and </title> tags, leaving just the raw text inside
                // We don't want to print <title>Rust</title> , we just want Rust . This extracts the text inside the tags
                let title_text = title_element.inner_html();
                println!("Success! Found the Title : {}", title_text);
            } else {
                println!("No <title> tag found on this page.");
            }
        },
        Err(e) => println!("Final Failure: {}",e),
    }
}