use error_chain::error_chain;
use std::collections::HashMap;
use std::time::Instant;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
