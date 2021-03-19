use crate::{GenericError, exists};

#[derive(Clone, Deserialize)]
pub struct CcConfig {
    pub ofbiz_loc: String,
}

impl CcConfig{
    pub fn load() -> Self {
        if exists("cc.toml") {
            let cnt = std::fs::read_to_string("cc.toml").expect("read cc.config");
            toml::from_str(cnt.as_str()).expect("toml conf")
        }else{
            let cnt=include_str!("cc.toml");
            toml::from_str(cnt).expect("embed conf")
        }
    }
}

lazy_static! {
    static ref CC_CONF: CcConfig = CcConfig::load() ;
}

pub fn cc_conf() -> Result<&'static CcConfig, GenericError>{
    Ok(&CC_CONF)
}
