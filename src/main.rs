mod connector;

use connector::Connector;
use hyper::{Body, Request};

fn main() {
    println!("Hello, world!");

    let mut connector = Connector::<dyn Fn(Body)>::default();
    connector.handler(|body: Body| {});
    connector.push_request(
        Request::builder()
            .method("GET")
            .uri("http://naver.com")
            .body(Body::empty())
            .unwrap(),
    );
    connector.push_request(
        Request::builder()
            .method("GET")
            .uri("http://daum.net")
            .body(Body::empty())
            .unwrap(),
    );
    connector.push_request(
        Request::builder()
            .method("GET")
            .uri("http://google.com")
            .body(Body::empty())
            .unwrap(),
    );

    let _ = connector.connect();
}
