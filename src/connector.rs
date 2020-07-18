use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

pub struct Connector {
    uris: Vec<Uri>,
}

impl Connector {
    pub fn new() -> Connector {
        Connector { uris: Vec::new() }
    }

    pub fn push_uri(&mut self, target_uri: Uri) {
        (&mut self.uris).push(target_uri)
    }

    #[tokio::main]
    pub async fn connect(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let mut futs = Vec::new();
        for uri in self.uris {
            let fut = async {
                let resp = client.get(uri).await?;
                println!("res : {}", resp.status());
                hyper::body::to_bytes(resp.into_body()).await
            };
            futs.push(fut);
        }

        for fut in futs {
            let _ = fut.await;
        }

        Ok(())
    }
}

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
    // let client = Arc::new(Client::new());

    let mut uris = Vec::new();
    uris.push("https://naver.com");
    uris.push("https://daum.net");
    uris.push("https://google.com");

    for uri_str in uris {
        // let client = client.clone();
        let uri = uri_str.parse()?;

        tokio::spawn(async move {
            let client = Client::new();
            let resp = client.get(uri).await;
            println!("RESP : {:?}", resp);
        });
    }

    // let uri = "http://naver.com".parse()?;
    // let resp = client.get(uri).await?;

    // println!("2 Response: {}", resp.status());

    // let uri = "http://daum.net".parse()?;
    // let resp = client.get(uri).await?;

    // println!("3 Response : {}", resp.status());

    // let uri = "http://google.com".parse()?;
    // let resp = client.get(uri).await?;

    // println!("4 Response : {}", resp.status());

    Ok(())
}
