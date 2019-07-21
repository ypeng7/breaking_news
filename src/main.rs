extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate futures;
extern crate select;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Stream};
use hyper_tls::HttpsConnector;
use futures::{future, Future};
use select::document::Document;
use select::predicate::Class;

fn main() {
    tokio::run(future::lazy(|| {

        // 4 is number of blocking DNS threads
        let https = HttpsConnector::new(4).unwrap();
        // let client = Client::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        let uri = "https://s.weibo.com/top/summary?cate=realtimehot".parse().unwrap();

        client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status());
                res
                    .into_body()
                    .for_each(|chunk| {
                        io::stdout()
                            .write_all(&chunk)
                            .map_err(|e| {
                                panic!("example expects stdout is open, error={}", e)
                            })
                    })
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
        }))
}
