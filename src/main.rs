/**
 * File              : src/main.rs
 * Author            : Yue Peng <yuepaang@gmail.com>
 * Date              : 2019-07-22 17:11:36
 * Last Modified Date: 2019-07-23 15:12:55
 * Last Modified By  : Yue Peng <yuepaang@gmail.com>

 */
extern crate reqwest;
extern crate select;

mod color;

use crate::color::Colorized;

use select::document::Document;
use select::predicate::{Name};

fn is_ad(text: String) -> bool {
    let text_ref: &str = &text;
    if text_ref == "javascript:void(0);" {
        true
    } else {
        false
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let requests_url = "https://s.weibo.com/top/summary?cate=realtimehot";
    let mut news = Vec::new();
    let mut urls = Vec::new();
    let resp = reqwest::get(requests_url).await?.text().await?;
    let document = Document::from_read(resp.as_bytes()).unwrap();
    document
        .find(Name("a"))
        .for_each(|x| news.push(x.text()));
    document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| urls.push(x.to_string()));

    println!("微博热搜榜Top20");
    println!("----------");
    let mut cnt = 0;
    for i in 0..news.len() {
        if cnt == 20 {
            break;
        }
        if i < 4 {
            continue;
        }
        if is_ad(String::from(urls.get(i).unwrap())) {
            continue;
        }
        let topic = format!("{}. {}", i-3, news.get(i).unwrap()).to_string().on_color("yellow");
        println!("{}", topic);
        // println!("{}. {}", i-3, news.get(i).unwrap());
        println!("https://s.weibo.com{}", urls.get(i).unwrap());
        println!("\n");
        cnt += 1;
    }

    let size: i32 = 10;
    let star = "*".red();
    for x in 0..10 {
        for y in 0..4*10 {
            let dist1 = (((x - size).pow(2) + (y - size).pow(2)) as f64).sqrt();
            let dist2 = (((x - size).pow(2) + (y - 3*size).pow(2)) as f64).sqrt();

            if dist1 < (size as f64) + 0.5 || dist2 < (size as f64) + 0.5 {
                print!("{}", star);
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    for x in 1..2*size {
        for _y in 0..x {
            print!(" ");
        }

        for _y in 0..(4*size+1 -2*x) {
            print!("{}", star);
        }
        print!("\n");
    }

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

