use std::error::Error;
use std::fmt;
use std::future::Future;
use std::time::Duration;

use std::fs;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Request, Response};
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;

pub struct TimeoutSet {
    connect: Option<Duration>,
    read: Option<Duration>,
    write: Option<Duration>,
}

impl Default for TimeoutSet {
    fn default() -> Self {
        TimeoutSet::new(None, None, None)
    }
}

impl TimeoutSet {
    pub fn new(
        connect: Option<Duration>,
        read: Option<Duration>,
        write: Option<Duration>,
    ) -> TimeoutSet {
        TimeoutSet {
            connect,
            read,
            write,
        }
    }
}

pub trait SaveFileObserver {
    fn on_save(&self, file: &fs::File);
}

pub struct Craft {
    client: Client<TimeoutConnector<HttpsConnector<HttpConnector>>>,
}

impl Default for Craft {
    fn default() -> Self {
        Craft::new(TimeoutSet::default())
    }
}

impl Craft {
    pub fn new(timeout: TimeoutSet) -> Craft {
        let https = HttpsConnector::new();
        let mut connector = TimeoutConnector::new(https);
        connector.set_connect_timeout(timeout.connect);
        connector.set_read_timeout(timeout.read);
        connector.set_write_timeout(timeout.write);

        let client = Client::builder().build::<_, Body>(connector);
        Craft { client }
    }

    pub async fn visit<V, HF>(
        &self,
        request: Request<Body>,
        response_handler: &HF,
    ) -> Result<V, CraftError>
    where
        HF: Fn(Response<Body>) -> Result<V, CraftError>,
    {
        let resp = (&self.client)
            .request(request)
            .await
            .map_err(|_err| CraftError::HyperConnector)?;

        response_handler(resp)
    }

    pub async fn visit_all<'a, V, HF>(
        &'a self,
        requests: Vec<Request<Body>>,
        response_handler: HF,
    ) -> Vec<Result<V, CraftError>>
    where
        HF: Fn(Response<Body>) -> Result<V, CraftError>,
    {
        let mut response_results = Vec::new();
        for req in requests {
            let res = self.visit(req, &response_handler);
            response_results.push(res);
        }

        futures::future::join_all(response_results).await
    }

    pub async fn handle_all_results<'a, V, F, RH>(
        &'a self,
        results: Vec<Result<V, CraftError>>,
        result_handler: RH,
    ) -> CraftResult
    where
        F: Future,
        RH: Fn(usize, Result<V, CraftError>) -> F,
    {
        let mut index: usize = 0;
        let mut handled_results = Vec::new();
        for result in results {
            handled_results.push(result_handler(index, result));
            index = index + 1;
        }

        let _ = futures::future::join_all(handled_results).await;
        Ok(())
    }

    pub async fn body_to_string(body: Body) -> Result<String, CraftError> {
        hyper::body::to_bytes(body)
            .await
            .map(|bytes| bytes.to_vec())
            .map(|vec| String::from_utf8(vec).unwrap_or("".to_owned()))
            .map_err(|_err| CraftError::HyperBodyHandling)
    }

    pub async fn save_body(
        mut body: Body,
        file_name: &str,
        file_observer: Option<Box<dyn SaveFileObserver>>,
    ) -> CraftResult {
        use hyper::body::HttpBody;
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

        let observer = file_observer.as_ref();

        let mut file = fs::File::create(file_name).map_err(|_err| CraftError::FileHandling)?;
        let mut ret_val = Ok(());
        'stream: while let Some(piece) = body.data().await {
            let save_result = piece
                .map_err(|_err| Error::new(ErrorKind::Other, "response body chunk error"))
                .and_then(|chunk| file.write_all(&chunk));

            if let Some(callback) = observer {
                callback.on_save(&file);
            }

            match save_result {
                Ok(_) => continue,
                Err(_err) => {
                    ret_val = Err(CraftError::FileHandling);
                    break 'stream;
                }
            }
        }

        ret_val
    }

    pub async fn join_all<I>(i: I)
    where
        I: IntoIterator,
        I::Item: Future,
    {
        futures::future::join_all(i).await;
    }
}

type CraftResult = Result<(), CraftError>;

#[derive(Debug)]
pub enum CraftError {
    HyperConnector,
    HyperBodyHandling,
    FileHandling,
    WrongResponseHandling,
}

impl fmt::Display for CraftError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CraftError::HyperConnector => write!(formatter, "HyperConnector"),
            CraftError::HyperBodyHandling => write!(formatter, "HyperBodyHandling"),
            CraftError::FileHandling => write!(formatter, "FileHandling"),
            CraftError::WrongResponseHandling => write!(formatter, "WrongResponseHandling"),
        }
    }
}

impl Error for CraftError {
    fn description(&self) -> &str {
        match *self {
            CraftError::HyperConnector => "HyperConnector",
            CraftError::HyperBodyHandling => "HyperBodyHandling",
            CraftError::FileHandling => "FileHandling",
            CraftError::WrongResponseHandling => "WrongResponseHandling",
        }
    }
}
