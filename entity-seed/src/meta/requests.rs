use std::collections::HashMap;

/// https://github.com/seanmonstar/reqwest
#[tokio::test]
async fn req_httpbin() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
