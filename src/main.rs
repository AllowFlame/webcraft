mod connector;
mod craft;

use connector::Connector;
use craft::Craft;
use hyper::{Body, Request};
use std::future::Future;

fn main() {
    println!("Hello, world!");

    let _ = run1();

    // let mut connector = Connector::default();
    // connector.handler(|index: usize, _body: Body| {
    //     println!("in callback index : {}", index);
    // });
    // connector.push_request(
    //     Request::builder()
    //         .method("GET")
    //         .uri("https://cdn.hiyobi.me/data/json/1681767_list.json")
    //         .body(Body::empty())
    //         .unwrap(),
    // );
    // connector.push_request(
    //     Request::builder()
    //         .method("GET")
    //         .uri("http://daum.net")
    //         .body(Body::empty())
    //         .unwrap(),
    // );
    // connector.push_request(
    //     Request::builder()
    //         .method("GET")
    //         .uri("http://google.com")
    //         .body(Body::empty())
    //         .unwrap(),
    // );

    // let _ = connector.connect();
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut craft = Craft::default();
    // let f = async move |index, body| {

    // };
    // let mut craft = Craft::new(|index, body| {});

    let req = Request::builder()
        .method("GET")
        .uri("https://cdn.hiyobi.me/data/json/1681767_list.json")
        .body(Body::empty())
        .unwrap();

    let body = craft.visit(req).await?;
    let s = Craft::body_to_string(body).await?;

    println!("out : {}", s.as_str());

    Ok(())
}

async fn handle(index: usize, body: Body) {
    let s = Craft::body_to_string(body).await.unwrap_or("".to_owned());

    println!("out : {}, {}", index, s.as_str());
}

#[tokio::main]
async fn run1() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let craft = Craft::default();

    let req = Request::builder()
        .method("GET")
        .uri("https://cdn.hiyobi.me/data/json/1681767_list.json")
        .body(Body::empty())
        .unwrap();

    let reqs = vec![req];

    craft.visit_all(reqs, handle).await?;

    // let body = craft.visit(req).await?;
    // let s = Craft::body_to_string(body).await?;

    // println!("out : {}", s.as_str());

    Ok(())
}
