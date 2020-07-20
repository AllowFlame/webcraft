mod connector;

use connector::Connector;
use hyper::{Body, Request};

fn main() {
    println!("Hello, world!");

    let mut connector = Connector::default();
    connector.handler(|index: usize, _body: Body| {
        println!("in callback index : {}", index);
    });
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
