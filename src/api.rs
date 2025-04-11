use chrono::Utc;
use reqwest::{
    Client, Error, Response,
    header::{CONTENT_TYPE, USER_AGENT},
};
use std::fs;

pub async fn request(client: &Client, url: &str, token: &str) -> Result<Response, Error> {
    let timestamp = Utc::now().to_rfc3339();
    let body = fs::read_to_string("./body.xml")
        .expect("Failed to open request `body.xml`!")
        .replace("NOW", &timestamp);
    let response = client
        .post(url)
        .bearer_auth(token)
        .header(CONTENT_TYPE, "application/xml")
        .header(USER_AGENT, "vbzli")
        .body(body)
        .send()
        .await;
    response
}
