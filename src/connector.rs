use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

pub struct Connector<H: Fn(Body)> {
    reqs: Vec<Request<Body>>,
    handle: Option<H>,
}

impl<H: Fn(Body)> Default for Connector<H> {
    fn default() -> Self {
        Connector {
            reqs: Vec::new(),
            handle: Option::None,
        }
    }
}

impl<H: Fn(Body)> Connector<H> {
    pub fn handler(&mut self, handle: H) {
        (&mut self).handle = Some(handle);
    }

    pub fn push_request(&mut self, request: Request<Body>) {
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
                let handle = self.handle.unwrap();

                println!("res : {}", resp.status());
                handle(resp.into_body());
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
