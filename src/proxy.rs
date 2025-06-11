use crate::jwt::generate_jwt;
use dotenvy::dotenv;
use hyper::body::to_bytes;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Request, Response, Uri};
use std::str;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    dotenv().ok();

    let (parts, body) = req.into_parts();
    let method = parts.method.clone();
    let uri = parts.uri.clone();
    let headers = parts.headers.clone();

    let whole_body = to_bytes(body).await?;
    let body_str = String::from_utf8_lossy(&whole_body);

    let dest_base = std::env::var("DEST_BASE").expect("DEST_BASE must be set");
    let path = uri.path_and_query().map(|x| x.as_str()).unwrap_or("/");
    let dest_uri: Uri = format!("{}{}", dest_base, path).parse().unwrap();

    // Generate JWT token to inject
    let jwt = generate_jwt();

    let mut builder = Request::builder()
        .method(method.clone())
        .uri(dest_uri.clone());

    if let Some(hdrs) = builder.headers_mut() {
        // Set content type (you can customize this)
        hdrs.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        // Inject JWT in custom header X-Proxy-Authorization
        hdrs.insert(
            "X-Proxy-Authorization",
            format!("Bearer {}", jwt).parse().unwrap(),
        );

        // Forward all other headers except the original Authorization and Content-Type
        for (k, v) in headers.iter() {
            if k != AUTHORIZATION && k != CONTENT_TYPE {
                hdrs.insert(k, v.clone());
            }
        }
    }

    println!("--- Incoming Proxy Request ---");
    println!("Method: {}", method);
    println!("URI: {}", uri);
    println!("Headers:");
    for (k, v) in headers.iter() {
        if k == AUTHORIZATION {
            println!("  {}: <redacted>", k);
        } else {
            println!("  {}: {:?}", k, v);
        }
    }
    println!("Body: {}", body_str);

    if let Some(hdrs) = builder.headers_ref() {
        println!("--- Outgoing Proxied Request Headers ---");
        for (k, v) in hdrs.iter() {
            println!("  {}: {:?}", k, v);
        }
    }

    let proxied = builder.body(Body::from(whole_body)).unwrap();
    println!("Forwarding request to: {}", dest_uri);

    let client = Client::new();
    let response = client.request(proxied).await?;

    println!("--- Response from downstream ---");
    println!("Status: {}", response.status());
    println!("Headers:");
    for (k, v) in response.headers().iter() {
        println!("  {}: {:?}", k, v);
    }

    let (parts, body) = response.into_parts();
    let body_bytes = to_bytes(body).await?;
    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Body: {}", body_str);

    let new_response = Response::from_parts(parts, Body::from(body_bytes));
    Ok(new_response)
}
