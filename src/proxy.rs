use hyper::{Body, Client, Request, Response, Uri};
use hyper::body::to_bytes;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use crate::jwt::generate_jwt;
use dotenvy::dotenv;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    dotenv().ok();

    let dest_base = std::env::var("DEST_BASE").expect("DEST_BASE must be set");
    let path = req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("/").to_string();
    let dest_uri: Uri = format!("{}{}", dest_base, path).parse().unwrap();
    let method = req.method().clone();
    let headers = req.headers().clone();
    let whole_body = to_bytes(req.into_body()).await.unwrap();
    let jwt = generate_jwt();
    

    let mut builder = Request::builder()
        .method(method)
        .uri(dest_uri);

    if let Some(hdrs) = builder.headers_mut() {
        hdrs.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        hdrs.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());
        for (k, v) in headers.iter() {
            if k != AUTHORIZATION && k != CONTENT_TYPE {
                hdrs.insert(k, v.clone());
            }
        }
    }
    
    let proxied = builder.body(Body::from(whole_body)).unwrap();
    let client = Client::new();

    client.request(proxied).await
}
