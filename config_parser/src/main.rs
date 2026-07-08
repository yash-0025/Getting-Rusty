// We take in a string and a prefix . We want to return the string without the prefix
// Notice there are two input references, and one output references
fn strip_prefix(content: &str, prefix: &str) -> &str {
    content.strip_prefix(prefix).unwrap_or(content)
}

fn main() {
    println!("Hello, world!");
}
