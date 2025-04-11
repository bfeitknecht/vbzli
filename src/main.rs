use std::{error::Error, fs::read_to_string, thread::sleep, time::Duration};
use vbzli::api::*;
use vbzli::driver::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://api.opentransportdata.swiss/ojp20";
    let token = read_to_string("./token.secret")?;
    loop {
        match request(&client, &url, &token).await {
            Ok(resp) => display(resp),
            Err(e) => debug(e),
        };
        sleep(Duration::from_secs(30));
    }
}
