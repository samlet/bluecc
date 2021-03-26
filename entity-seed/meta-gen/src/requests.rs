use std::collections::HashMap;
use reqwest::{header, StatusCode as Status, Client};
use seed::{new_snowflake_id, GenericError};
use serde::{Serialize, Deserialize, de};
use crate::UserWithPassword;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SrvResp<T> {
    pub status_code: i32,
    pub status_description: String,
    #[serde(default)]
    pub success_message: String,
    pub data: Option<T>,
    // error part
    #[serde(flatten, default)]
    pub err: SrvErr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SrvErr{
    #[serde(default)]
    pub error_type: String,
    #[serde(default)]
    pub error_message: String,
    #[serde(default)]
    pub error_description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenData{
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
}

impl<T: Serialize> SrvResp<T>{
    pub fn is_ok(&self) -> bool {
        self.status_code==200
    }
    pub fn pretty_str(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Clone)]
pub struct SrvDeles{
    client: Client,
    pub access_token: String,
    pub user_login: UserWithPassword,
}
impl SrvDeles{
    pub fn new() -> Self {
        let mut client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build().unwrap();
        SrvDeles { client: (client), access_token: "".to_string(),
            user_login: UserWithPassword{ user: "admin".to_string(), password: "ofbiz".to_string() } }
    }

    pub async fn default_auth(&self) -> Result<SrvResp<TokenData>, GenericError> {
        let res = self.client
            .post("https://localhost:8443/rest/auth/token")
            // .header(header::AUTHORIZATION, "Basic YWRtaW46b2ZiaXo=")
            .header(header::AUTHORIZATION,
                    format!("Basic {}", self.user_login.encode()?))
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        debug!("result -> {} {:?}", res.status(), res);

        let data=res.json::<SrvResp<TokenData>>().await?;
        Ok(data)
    }

    pub async fn default_token(&self) -> Result<String, GenericError> {
        let data: SrvResp<TokenData>=self.default_auth().await?;
        let tok=if data.is_ok() {data.data.unwrap().access_token} else {"".to_string()};
        Ok(tok)
    }

     pub async fn use_default_token(&mut self) -> Result<(), GenericError> {
         self.access_token=self.default_token().await?;
         Ok(())
     }

    pub async fn srv<T,R>(&self, srv_name: &str, json_req: &T) -> Result<SrvResp<R>, GenericError>
    where T: Serialize + ?Sized,
          R: de::DeserializeOwned {
        let srv_url = format!("https://localhost:8443/rest/services/{}", srv_name);
        let res = self.client
            .post(srv_url.as_str())
            .header(header::AUTHORIZATION,
                    format!("Bearer {}",&self.access_token))
            .header(header::ACCEPT, "application/json")
            .json(json_req)
            .send()
            .await?;
        debug!("result -> {} {:?}", res.status(), res);
        let data = res.json::<SrvResp<R>>().await?;
        Ok(data)
    }
}

#[tokio::test]
async fn srv_auth_works() -> Result<(), GenericError> {
    let dele=SrvDeles::new();
    let data: SrvResp<TokenData>=dele.default_auth().await?;
    let data_json=serde_json::to_string_pretty(&data)?;
    println!("{}", data_json);
    let tok=if data.is_ok() {data.data.unwrap().access_token} else {"_".to_string()};
    println!("token: {}", tok);
    Ok(())
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

    let data=res.json::<SrvResp<TokenData>>().await?;
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

    let data=res.json::<SrvResp<SrvData>>().await?;
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

    let data=res.json::<SrvResp<SrvData>>().await?;
    let data_json=serde_json::to_string_pretty(&data)?;
    println!("{}", data_json);

    Ok(())
}


#[tokio::test]
async fn srv_invoke_works() -> Result<(), GenericError> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct SimpleReq {
        default_value: f64,
        message: String
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    struct SrvData{
        pub resp: String,
    }

    let mut dele=SrvDeles::new();
    dele.use_default_token().await?;
    println!("tok {}", dele.access_token);

    let ret: SrvResp<SrvData>=dele.srv("testScv", &SimpleReq{
            default_value: 1.0,
            message: "hello".to_string()
        }).await?;

    let data_json=serde_json::to_string_pretty(&ret)?;
    println!("{}", data_json);

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamicValue {
    #[serde(flatten)]
    pub values: HashMap<String, Value>,
}

#[tokio::test]
async fn srv_invoke_with_dynamic_works() -> Result<(), GenericError> {
    let mut dele=SrvDeles::new();
    dele.use_default_token().await?;
    println!("tok {}", dele.access_token);

    let mut values=HashMap::new();
    values.insert("defaultValue".to_string(), Value::from(8.8));
    values.insert("message".to_string(), Value::from("hi"));
    let ret: SrvResp<DynamicValue>=dele.srv("testScv", &DynamicValue{ values: values,}).await?;
    let data_json=serde_json::to_string_pretty(&ret)?;
    println!("{}", data_json);

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PartyValue {
    party_id: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[test]
fn party_value_works() -> anyhow::Result<()> {
    let json = json!({
          "partyId": "49824073-979f-4814-be10-5ea416ee1c2f",
          "username": "john_doe",
          "mascot": "Ferris"
        });
    let party:PartyValue=serde_json::from_value(json)?;
    println!("{:?}", party.extra);
    Ok(())
}


