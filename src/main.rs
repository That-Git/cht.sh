use {
    http_body_util::{BodyExt, Empty},
    hyper::{
        body::{Buf, Bytes},
        header::CONTENT_TYPE,
        http::HeaderMap,
        Request,
    },
    hyper_util::rt::TokioIo,
    std::io::Write,
    tokio::{
        io::{self, AsyncWriteExt as _},
        net::TcpStream,
    },
};

fn prompt(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}
// -> &'static str
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let topic: String = prompt("topic: ");
    let query: String = prompt("query: ");
    // Parse our URL...
    let url = (format!("https://cht.sh/{}~{}", topic.to_string(), query.to_string()))
        .as_str()
        .parse::<hyper::Uri>()?;

    // Get the host and the port
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);

    // Create the Hyper client
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // Spawn a task to poll the connection, driving the HTTP state
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });
    // The authority of our URL will be the hostname of the httpbin remote
    let authority = url.authority().unwrap().clone();

    // Create an HTTP request with an empty body and a HOST header
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .header(hyper::header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(hyper::header::USER_AGENT, "curl")
        .header(hyper::header::ACCEPT, "text/plain")
        .body(Empty::<Bytes>::new())?;

    // Await the response...
    let mut res = sender.send_request(req).await?;

    println!("Response status: {}", res.status());
    // Stream the body, writing each frame to stdout as it arrives
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(chunk).await?;
        }
    }

    Ok(())
}
