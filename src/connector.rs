use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

pub struct Connector<H: Fn(usize, Body)> {
    reqs: Vec<Request<Body>>,
    handle: Option<H>,
}

impl<H: Fn(usize, Body)> Default for Connector<H> {
    fn default() -> Self {
        Connector {
            reqs: Vec::new(),
            handle: Option::None,
        }
    }
}

impl<H: Fn(usize, Body)> Connector<H> {
    pub fn handler(&mut self, handle: H) {
        self.handle = Some(handle);
    }

    pub fn push_request(&mut self, request: Request<Body>) {
        (&mut self.reqs).push(request);
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
                hyper::Result::Ok(resp.into_body())
            };
            futs.push(fut);
        }

        let mut index: usize = 0;
        let handler = self.handle.as_ref();
        for fut in futs {
            async {
                let body = fut.await.unwrap_or(Body::default());
                handler.map(|handle| {
                    handle(index, body);
                });
            }
            .await;

            index = index + 1;
        }

        Ok(())
    }
}
