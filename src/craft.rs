use std::future::Future;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

pub struct Craft {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Default for Craft {
    fn default() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        Craft { client }
    }
}

impl Craft {
    pub async fn visit(&self, request: Request<Body>) -> hyper::Result<Body> {
        let resp = (&self.client).request(request).await?;
        hyper::Result::Ok(resp.into_body())
    }

    pub async fn visit_all<F: Future, H: Fn(usize, Body) -> F>(
        &self,
        requests: Vec<Request<Body>>,
        handler: H,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();
        for req in requests {
            let res = self.visit(req);
            results.push(res);
        }

        let mut index: usize = 0;
        let bodies = futures::future::join_all(results).await;
        for body in bodies {
            handler(index, body?).await;
            index = index + 1;
        }

        Ok(())
    }

    pub async fn body_to_string(body: Body) -> hyper::Result<String> {
        hyper::body::to_bytes(body)
            .await
            .map(|bytes| bytes.to_vec())
            .map(|vec| String::from_utf8(vec).unwrap_or("".to_owned()))
    }

    pub async fn save_body(mut body: Body, file_name: &str) {
        use hyper::body::HttpBody;
        use std::fs;
        use std::io::Write;
        use std::path::PathBuf;

        let path = PathBuf::from(file_name);
        path.parent().and_then(|parent_path| {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path).ok()
            } else {
                Some(())
            }
        });

        let mut file = fs::File::create(file_name).expect("file error");
        'stream: while let Some(piece) = body.data().await {
            let chunk = match piece {
                Ok(piece) => piece,
                Err(_err) => {
                    break 'stream;
                }
            };
            file.write_all(&chunk).unwrap();
        }
    }
}
