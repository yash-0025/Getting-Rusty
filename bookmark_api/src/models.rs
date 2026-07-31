use serde::{Deserialize, Serialize};

// The Database Row model [maps directly to SQLite table]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow )]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub tags: String,
    pub created_at: String,
}

// The DTO [Date Transfer Object ] for creating a new bookmark via POST request
#[derive(Debug, Deserialize)]
pub struct CreateBookmark {
    pub url: String,
    pub title: String,

// If the user omits description from the incoming JSON payload , serde automatically sets this field to NONE instead of throwing an error 
    pub description: Option<String>,
    pub tags: Option<String>,
}


// Query parameters DTO for searh request [GET /bookmarks/search?q=rust]
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}



// #[derive] - Rust procefural macro attribute that auto generates code for our struct
// Debug - Allows printing the struct with println!("{:?}")
// Serialize - Allow converting a Bookmark struct instance into a JSON string sent to the browser
// Deserialize - Allows converting incoming JSON strings into a Bookmark struct 
// sqlz::FromRow - The Magic Database Attribute Automatically maps raw database row columns (id, url, title) to rust struct fields by field name
