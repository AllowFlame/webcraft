mod connector;
mod craft;

use connector::Connector;
use craft::Craft;
use hyper::{Body, Request};
use hyper::body::HttpBody;
use std::future::Future;
use tokio::io::{self, AsyncWriteExt as _};

fn main() {
    println!("Hello, world!");

    let _ = run();
}

async fn handle(index: usize, mut body: Body) {
    // let s = Craft::body_to_string(body).await.unwrap_or("".to_owned());

    // println!("out : {}, {}", index, s.as_str());
    while let Some(next) = body.data().await {
        let chunk = next.unwrap();
        io::stdout().write_all(&chunk).await.unwrap();
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let craft = Craft::default();

    let req = Request::builder()
        .method("GET")
        .uri("https://cdn.hiyobi.me/data/json/1681767_list.json")
        // .uri("https://getmiso.com")
        .body(Body::empty())
        .unwrap();

    let reqs = vec![req];

    craft.visit_all(reqs, handle).await?;

    Ok(())
}

struct Te;

impl Te {
    async fn async_handle(index: usize, body: Body) {
        let s = Craft::body_to_string(body).await.unwrap_or("".to_owned());

        println!("Te::async_handle::out : {}, {}", index, s.as_str());
    }
}

trait Handler {
    fn handle(index: usize, body: Body) -> Future<Output = ()>;
}

// impl Handler for Te {
//     fn handle(index: usize, body: Body) -> Future<Output = ()> {
//         let s = Craft::body_to_string(body).await.unwrap_or("".to_owned());

//         println!("Te::handle::out : {}, {}", index, s.as_str());
//     }
// }
