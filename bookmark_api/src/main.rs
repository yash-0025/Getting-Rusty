use axum::{extract::State, routing::get, Router};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::sync::Arc;


mod models;


pub struct AppState {
    pub db: SqlitePool,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Bookmark API server...");


// Create SQLite Database File if it doesn't exist and connect to Pool

// sqlite:// is the database scheme. mode=rwc stands for Read-Write-Create. It tells SQLite. If bookmarks.db does not exist on disk, create it automactically
    let db_url = "sqlite://bookmarks.db?mode=rwc";

// Configure and create a connection pool of up to 5 concurrent connections
// max_connections(5) limits concurrent SQLite database connections to 5
// .await? - Asynchronously connects to the database (.await) and uses? to unwrap the Result , bubbling up any connection error to main

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
    
    println!("Database connection pool initialized.");


    // Automatically run pending SQL migration on startup
//  reads all the .sql files inside ./migrations at compile time.
// .run(&pool)  executes any new SQL scripts against SQLite automatically when the server boots up 
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
        
    println!("Database migrations applied successfully");


// Wraps our Appstate inside an Arc Atomic Reference Counting pointer so multiple async threads can safely hold references to the database pool
    // Wrap database pool inside shared Appstate using Arc
    let app_state = Arc::new(AppState {db:pool });

    // Build Axum Router with Health Check route
// Router::new() creates a new Axum router instance
// .route(...) = Maps GET /health requests to the health_check async function
// with_state(app_state) Injects app_state into the router so all routes can extract State<Arc<AppState>>
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(app_state);

    // Bind TCP listener to 127.0.0.1:3000
// Binds and async TCP socket listener to local IP 127.0.0.1 on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server listening on http://127.0.0.1:3000");

// Starts the main web server loop. It listens for incoming TCP connections and dispatches then to our Axum router .
    axum::serve(listener, app).await?;

    Ok(())
}

// Basic Health Check Handler
async fn health_check(State(state): State<Arc<AppState>>) -> &'static str {
    // Simple verification that we can acquire a connection from the pool
    let _conn = state.db.acquire().await;
    "OK - Bookmark API is healthy!"
}