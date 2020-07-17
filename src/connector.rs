use hyper::Client;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let resp = client.get(uri).await?;

    println!("1 Response: {}", resp.status());

    Ok(())
}

#[tokio::main]
pub async fn run1() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let uri = "http://naver.com".parse()?;
    let resp = client.get(uri).await?;

    println!("2 Response: {}", resp.status());

    let uri = "http://daum.net".parse()?;
    let resp = client.get(uri).await?;

    println!("3 Response : {}", resp.status());

    let uri = "http://google.com".parse()?;
    let resp = client.get(uri).await?;

    println!("4 Response : {}", resp.status());

    Ok(())
}
