mod connector;

use connector::Connector;

fn main() {
    println!("Hello, world!");

    // let _ = connector::run();
    // let _ = connector::run1();

    let mut connector = Connector::new();
    connector.push_uri("http://naver.com".parse().unwrap());
    connector.push_uri("http://daum.net".parse().unwrap());
    connector.push_uri("http://google.com".parse().unwrap());

    let _ = connector.connect();
}
