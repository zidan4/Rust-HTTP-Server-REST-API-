use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the endpoint
    let hello = warp::path("hello")
        .map(|| warp::reply::json(&"Hello, World!"));

    println!("Server running at http://localhost:3030/hello");
    
    // Start the server
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
