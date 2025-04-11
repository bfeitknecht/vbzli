use futures::executor::block_on;
use reqwest::{Error, Response};

pub fn display(resp: Response) -> () {
    match resp.status() {
        reqwest::StatusCode::OK => {
            let body = block_on(resp.text()).expect("Error awaiting response body!");
            println!("{}", body);
        }
        _ => {}
    };
}

pub fn debug(err: Error) -> () {
    let _ = err;
}
