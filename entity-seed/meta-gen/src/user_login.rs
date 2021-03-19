use base64::{encode, decode, DecodeError};
use bstr::{B, ByteSlice};
use std::borrow::Cow;
use crate::GenericError;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserWithPassword {
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
}

impl UserWithPassword{

    pub fn encode(&self) -> Result<String, GenericError>{
        Ok(encode(format!("{}:{}", self.user, self.password).as_str()))
    }

    pub fn decode(&mut self, b64_str: &str) -> Result<&Self, DecodeError>{
        let login=decode(b64_str)?;
        let parts: Vec<&[u8]>=login.split_str(":").collect();
        self.user= parts[0].to_str_lossy().to_string();
        self.password= parts[1].to_str_lossy().to_string();
        Ok(self)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base64_works() -> anyhow::Result<()> {
        let a = b"hello world";
        let b = "aGVsbG8gd29ybGQ=";

        assert_eq!(encode(a), b);
        assert_eq!(a, &decode(b).unwrap()[..]);

        let login = decode("YWRtaW46b2ZiaXo=")?;
        println!("{:?}", login.to_str_lossy());
        assert_eq!(Cow::Borrowed("admin:ofbiz"), login.to_str_lossy());
        assert_eq!("YWRtaW46b2ZiaXo=", encode("admin:ofbiz"));

        // user-login
        let mut login = UserWithPassword::default();
        println!("{:?}", login.decode("YWRtaW46b2ZiaXo=")?);
        Ok(())
    }
}


