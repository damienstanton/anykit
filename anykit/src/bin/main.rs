//! A boilerplate CLI.
use std::{
    collections::HashMap,
    env,
    net::Ipv4Addr
};

use warp::{http::Response, Filter};
use anykit::math;

#[tokio::main]
async fn main() {
    assert!(math::add(1, 1) > 1);
    let x = warp::get()
        .and(warp::path("api"))
        .and(warp::path("AnyFunc"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });
        let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
        let port: u16 = match env::var(port_key) {
            Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
            Err(_) => 3000,
        };
    
    warp::serve(x).run((Ipv4Addr::UNSPECIFIED, port)).await
}