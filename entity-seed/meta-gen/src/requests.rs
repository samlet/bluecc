use std::collections::HashMap;
use reqwest::{header, StatusCode as Status};
use seed::{new_snowflake_id, GenericError};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SimpleResp<T> {
    pub status_code: i32,
    pub status_description: String,
    #[serde(default)]
    pub success_message: String,
    pub data: Option<T>,
    // error part
    #[serde(default)]
    pub error_type: String,
    #[serde(default)]
    pub error_message: String,
    #[serde(default)]
    pub error_description: String,
}

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

#[tokio::test]
async fn auth_works() -> Result<(), GenericError> {
    // let client = reqwest::Client::new();
    let mut client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build().unwrap();
    let res = client
        .post("https://localhost:8443/rest/auth/token")
        .header(header::AUTHORIZATION, "Basic YWRtaW46b2ZiaXo=")
        .header(header::ACCEPT, "application/json")
        .send()
        .await?;
    println!("result -> {} {:?}", res.status(), res);

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct TokenData{
        pub access_token: String,
        pub token_type: String,
        pub expires_in: String,
    }
    let data=res.json::<SimpleResp<TokenData>>().await?;
    let data_json=serde_json::to_string_pretty(&data)?;
    println!("{}", data_json);
    Ok(())
}

const ACCESS_TOKEN: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1c2VyTG9naW5JZCI6ImFkbWluIiwiaXNzIjoiQXBhY2hlT0ZCaXoiLCJleHAiOjE2MTYwNjc0NTUsImlhdCI6MTYxNjA2NTY1NX0.6-k_8xJ5vaTsTdD5xJLiSaBDkLCV4BdtMew50Hcte3dlaY0uPgMTxo491tcMSAbA15GfSFsJUkNBTBH0VsfNDg";

#[tokio::test]
async fn test_scv_works() -> Result<(), GenericError> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct SimpleReq {
        default_value: f64,
        message: String
    }
    // let client = reqwest::Client::new();
    let mut client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build().unwrap();

    let res = client
        .post("https://localhost:8443/rest/services/testScv")
        .header(header::AUTHORIZATION, ACCESS_TOKEN)
        .header(header::ACCEPT, "application/json")
        .json(&SimpleReq{
            default_value: 1.0,
            message: "hello".to_string()
        })
        .send()
        .await?;
    println!("result -> {} {:?}", res.status(), res);

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    struct SrvData{
        pub resp: String,
    }

    let data=res.json::<SimpleResp<SrvData>>().await?;
    let data_json=serde_json::to_string_pretty(&data)?;
    println!("{}", data_json);

    Ok(())
}

#[tokio::test]
async fn test_fail_srv_works() -> Result<(), GenericError> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct SimpleReq {
        default_value: f64,
        message: String
    }
    // let client = reqwest::Client::new();
    let mut client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build().unwrap();

    let res = client
        .post("https://localhost:8443/rest/services/testEntityFailure")
        .header(header::AUTHORIZATION, ACCESS_TOKEN)
        .header(header::ACCEPT, "application/json")
        .json(&SimpleReq{
            default_value: 1.0,
            message: "hello".to_string()
        })
        .send()
        .await?;
    println!("result -> {} {:?}", res.status(), res);

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    struct SrvData{
        pub resp: String,
    }

    let data=res.json::<SimpleResp<SrvData>>().await?;
    let data_json=serde_json::to_string_pretty(&data)?;
    println!("{}", data_json);

    Ok(())
}

