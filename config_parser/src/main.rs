// We take in a string and a prefix . We want to return the string without the prefix
// Notice there are two input references, and one output references
// fn strip_prefix(content: &str, prefix: &str) -> &str {
//     content.strip_prefix(prefix).unwrap_or(content)
// }


// 1. We declare the lifetime <'a>
// 2. We tag content as lasting for 'a
// 3. We tag the return type as lasting for 'a
// We do not tag prefix , because the output is NOT borrowed from it

fn strip_prefix<'a>(content: &'a str, prefix: &str) -> &'a str {
    content.strip_prefix(prefix).unwrap_or(content)
}

fn main() {
    println!("Hello, world!");
}
