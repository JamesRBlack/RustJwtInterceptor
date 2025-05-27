use hyper::body::Body;
use hyper::client::Client;
use hyper::{Request, Response, Uri};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use crate::jwt::generate_jwt;

const DEST_BASE: &str = "https://open-domain.com/proxy/ai.leo-james.com";

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("/");
    let dest_uri: Uri = format!("{}{}", DEST_BASE, path).parse().unwrap();

    let jwt = generate_jwt();

    let mut builder = Request::builder()
        .method("POST")
        .uri(dest_uri);

    if let Some(headers) = builder.headers_mut() {
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, format!("Bearer {}", jwt).parse().unwrap());
    }

    let proxied = builder.body(req.into_body()).unwrap();
    let client = Client::new();

    client.request(proxied).await
}
