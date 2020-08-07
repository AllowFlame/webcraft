use std::future::Future;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

pub struct Craft<T> {
    client: Client<HttpsConnector<HttpConnector>>,
    tagger: Option<T>,
}

impl<T> Default for Craft<T> {
    fn default() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        Craft {
            client,
            tagger: None,
        }
    }
}

impl<T> Craft<T> {
    pub async fn visit(&self, request: Request<Body>) -> hyper::Result<Body> {
        let resp = (&self.client).request(request).await?;
        hyper::Result::Ok(resp.into_body())
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
        use std::io::{Error, ErrorKind};
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
            let save_result = piece
                .map_err(|_err| Error::new(ErrorKind::Other, "response body chunk error"))
                .and_then(|chunk| file.write_all(&chunk));

            match save_result {
                Ok(_) => continue,
                Err(_err) => break 'stream,
            }
        }
    }
}

impl<T> Craft<T>
where
    T: Fn() -> String,
{
    pub fn new(tagger: Option<T>) -> Craft<T> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        Craft { client, tagger }
    }

    pub async fn visit_all<'a, F: Future, H: Fn(usize, Body, Option<&'a T>) -> F>(
        &'a self,
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
            handler(index, body?, self.tagger.as_ref()).await;
            index = index + 1;
        }

        Ok(())
    }
}
