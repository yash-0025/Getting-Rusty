// We take in a string and a prefix . We want to return the string without the prefix
// Notice there are two input references, and one output references
// fn strip_prefix(content: &str, prefix: &str) -> &str {
//     content.strip_prefix(prefix).unwrap_or(content)
// }

// The <'a> is the sticky note on the struct
pub struct Config<'a> {
    // We store a vector of tuples. Each tuple is (key value)
    // Both the key and the value are string slices that live for 'a
    pub entries: Vec<(&'a str, &'a str)>,
}

impl<'a> Config<'a> {
    // the parser takes a string slice and returns a config
    // Thanks to lifetime elision rust konws the returned config is pointing at document
    pub fn parse(document: &'a str) -> Self {
        let mut entries = Vec::new();

        // .lines() gives us an iterator that goes line by line
        for line in document.lines() {
            // Skip empty lines or comments
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            // .split_once('=') cuts the line into two pieces at the '='
            if let Some((key, value)) = line.split_once('=') {
                // We trin whitespace and push the exact string slices
                // Zero allocations [No string::from, bo.clone()]
                entries.push((key.trim(), value.trim()));
            }
        }

        Config { entries }

    }
}

// 1. We declare the lifetime <'a>
// 2. We tag content as lasting for 'a
// 3. We tag the return type as lasting for 'a
// We do not tag prefix , because the output is NOT borrowed from it

fn strip_prefix<'a>(content: &'a str, prefix: &str) -> &'a str {
    content.strip_prefix(prefix).unwrap_or(content)
}

fn main() {
    // A mock config file loaded into memory 
    let file_content = "
    # Database Settings
    host = localhost
    port = 5432

    # User Settings
    theme = dark
    " ;

    let my_config = Config::parse(file_content);

    println!("--- Parsed Configuration ---");
    for (key, value) in my_config.entries {
        println!("Key: {} -> Value: {}", key,value);
    }
}
