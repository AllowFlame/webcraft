use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

pub struct Connector {
    reqs: Vec<Request<Body>>,
}

impl Connector {
    pub fn new() -> Connector {
        Connector { reqs: Vec::new() }
    }

    pub fn push_uri(&mut self, request: Request<Body>) {
        (&mut self.reqs).push(request)
    }

    #[tokio::main]
    pub async fn connect(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        let mut futs = Vec::new();
        for req in self.reqs {
            let fut = async {
                let resp = client.request(req).await?;
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
