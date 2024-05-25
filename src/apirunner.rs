use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: hyper::Uri) -> Result<ApiResponse> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let req = Request::builder()
        .method("GET")
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    let mut response_body = String::new();
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            response_body +=
                std::str::from_utf8(&chunk).unwrap_or_else(|_| "Failed to parse body".into());
        }
    }

    Ok(ApiResponse::new(
        response_body,
        res.status().as_u16(),
        res.headers()
            .into_iter()
            .map(|h| {
                (
                    h.0.to_string(),
                    h.1.to_str().expect("Cannot parse header").to_string(),
                )
            })
            .collect(),
    ))
}

pub struct ApiResponse {
    pub body: String,
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
}

impl ApiResponse {
    fn new(body: String, status_code: u16, headers: Vec<(String, String)>) -> ApiResponse {
        ApiResponse {
            body,
            status_code,
            headers,
        }
    }
}
