use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};

mod jwt;
mod proxy;
mod types;

#[tokio::main]
async fn main() {
    // We'll listen on localhost:3000
    let addr = ([127, 0, 0, 1], 3000).into();

    // Create a service that clones no state, just calls our handler
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(proxy::handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run the server and await termination
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
