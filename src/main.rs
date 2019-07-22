/**
 * File              : src/main.rs
 * Author            : Yue Peng <yuepaang@gmail.com>
 * Date              : 2019-07-22 17:11:36
 * Last Modified Date: 2019-07-22 17:11:36
 * Last Modified By  : Yue Peng <yuepaang@gmail.com>

 */
extern crate reqwest;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let requests_url = "https://s.weibo.com/top/summary?cate=realtimehot";
    let body = reqwest::get(requests_url)?
        .text()?;
    println!("{:?}", body);
    Ok(())
}

// #![feature(async_await)]
// #![deny(warnings)]
// extern crate hyper;
// extern crate pretty_env_logger;

// use std::env;
// use std::io::{self, Write};

// use hyper::Client;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// #[hyper::rt::main]
// async fn main() -> Result<()> {
//     pretty_env_logger::init();

//     let url = match env::args().nth(1) {
//         Some(url) => url,
//         None => {
//             println!("Usage: client <url>");
//             return Ok(());
//         }
//     };

//     let url = url.parse::<hyper::Uri>().unwrap();
//     if url.scheme_part().map(|s| s.as_ref()) != Some("http") {
//         println!("Only works with 'http' URLs.");
//         return Ok(());
//     }

//     fetch_url(url).await
// }

// async fn fetch_url(url: hyper::Uri) -> Result<()> {
//     let client = Client::new();

//     let response = client.get(url).await?;

//     println!("Response: {}", response.status());
//     println!("Headers: {:#?}\n", response.headers());

//     let mut body = response.into_body();

//     while let Some(next) = body.next().await {
//         let chunk = next?;
//         io::stdout().write_all(&chunk)?;
//     }

//     println!("\n\nDone!");

//     Ok(());
// }

// fn main() {

//     rt::run(rt::lazy(|| {

//         // 4 is number of blocking DNS threads
//         let https = HttpsConnector::new(4).expect("TLS initialization failed");
//         let client = Client::builder()
//             .build::<_, hyper::Body>(https);

//         let url = "https://s.weibo.com/top/summary?cate=realtimehot".parse().unwrap();


//         client
//             .get(url)
//             .map(|res| {
//                 println!("Response: {}", res.status());
//                 println!("{:?}", res);
//                 res.into_body()
//                 // res
//                 //     .into_body()
//                 //     .for_each(|chunk| {
//                 //         io::stdout()
//                 //             .write_all(&chunk)
//                 //             .map_err(|e| {
//                 //                 panic!("example expects stdout is open, error={}", e)
//                 //             })
//                 //     })
//             })
//             .map_err(|err| {
//                 println!("Error: {}", err);
//             })
//         }))
// }
