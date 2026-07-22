use::std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use scraper::{Html, Selector}; // Import our HTML parser
use tokio::sync::Semaphore;
use tokio::time::{sleep, timeout};



// This is a helper function just like main it returns a Future that must be .await ed 
// Helper function that tries to fetch a URL with a retry loop
async fn fetch_and_parse_title(url: &str, max_retries: u32) -> Result<String, String> {

    let mut attempts = 0;

    let title_selector = Selector::parse("title").unwrap();

    while attempts < max_retries {

        // We wrapt the entire requesst in a 5-second timeout
        let fetch_result = timeout(Duration::from_secs(5), reqwest::get(url)).await;

        // println!("Attempt {}/{} for {}", attempts + 1, max_retries, url);
    
        match fetch_result {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    let html = response.text().await.map_err(|e| e.to_string())?;

                    let document = Html::parse_document(&html);

                    if let Some(title_element) = document.select(&title_selector).next() {
                        // return the title text
                        return Ok(title_element.inner_html());
                    } else {
                        return Ok("No Title Found".to_string());
                    }
                }
            },
            Ok(Err(e)) => println!("Network error for {}: {}", url, e),
            Err(_) => println!("Timeout waiting for {} (took > 5s)", url),
        }


    //     // Try the network request
    // match reqwest::get(url).await {
    //     Ok(response) => {
    //         if response.status().is_success() {

    //             // EXtract the HTML body as a string
    //             // Earlier we just printed response.status(). Now we actually want the website data The .text() method download the HTML body. Notice it has a .await on it
    //             // That's because downloading a large 10 MB HTML file from a scoket takes time so that operation is also asynchronous

    //             let html = response.text().await.unwrap();

    //             // If the request succeds we immediately exit the while loop and returns the HTML string
    //             return Ok(html);
    //         } else {
    //             println!("Server returned error: {}", response.status());
    //         }
    //     },
    //     Err(e) => {
    //         println!("Network Error: {}", e);
        // }
    // }

    attempts += 1;

    if attempts < max_retries {
        // println!("Sleeping for 3 seconds before next attenpt ..\n");

        // We use tokio::time::sleep . If we used std::thread::sleep here
        // we would freeze the entire tokio executor
        // The kitchen timer. We wait 3 seconds without blocking the OS thread 
        sleep(Duration::from_secs(3)).await;
        }
    }

    // Err(format!("Failed to fetch {} after {} attempts.", url, max_retries))
    Err("Failed after max retries".to_string())
}

#[tokio::main]
async fn main() {
    // Intentionally test a fake url that doesn't exist to trigger the retries
    // let bad_url = "https://this-website-definitely-does-not-exist-12345.com";

    // let url = "https://www.rust-lang.org/";

    let urls = vec![
        "https://www.rust-lang.org/",
        "https://github.com",
        "https://google.com",
        "https://this-website-definitely-does-not-exist-12345.com",
    ];

    // match fetch_with_retry(bad_url, 3).await {
    //     Ok(html) => println!("Success! Downloaded {} bytes of HTML.", html.len()),
    //     Err(e) => println!("Final Failure: {}", e),
    // }

    let semaphore = Arc::new(Semaphore::new(2)); // Max 2 concurrent requests

    let mut handles = vec![] ;

    // Create ore overwrite our CSV files and write the header now 
    // Opens a new file in the current directory . If it already exists it completely overwrites it 
    let mut file = File::create("results.csv").expect("Could not create file");

    // Just like println! but instead of printing to the terminal it write the formatted string into the File we just opened and automatically adds a newline \n at the end.
    writeln!(file, "URL,Title,Status").expect("Could not write header");

    for url in urls {
        let sem_clone = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();

            // Call our resilient scraper function
            let result = fetch_and_parse_title(url, 3).await;

            // format the result as a CSV row
            let csv_row = match result {

                // the format! macro works exactly like println! but it returns a String instead of printing it . We use \" to escape double quotes so they dont' break our string literal . This ensures our CSV handles commas inside the title correctly
                // title.replace() - In CSV format if a title contains a quote we must double it to escape it properly
                Ok(title) => format!("{},\"{}\",SUCCESS", url, title.replace("\"", "\"\"")), 
                Err(e) => format!("{},\"{}\", FAILED",url,e),
            };

            // Notice how our tokio::spawn closure returns a String? This means handle.await.unwrap() will gives us that exact string back
            return csv_row;
        });

        handles.push(handle);
    }

    // Wait for all task to finish and write their reslts to the file
    for handle in handles {
        let csv_row = handle.await.unwrap();
        // we write to the file synchronously at the very end
        writeln!(file, "{}", csv_row).expect("Could not write row");
    }

    println!("Scraping Complete! Check results.csv");



    /* match fetch_with_retry(url, 3).await {
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
    } */
}