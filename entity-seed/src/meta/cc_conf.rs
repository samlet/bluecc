use crate::GenericError;

#[derive(Clone, Deserialize)]
pub struct CcConfig {
    pub ofbiz_loc: String,
}

lazy_static! {
    static ref CC_CONF: CcConfig = {
        let cnt=std::fs::read_to_string("cc.toml").expect("read cc.config");
        toml::from_str(cnt.as_str()).expect("toml conf")
    };
}

pub fn cc_conf() -> Result<&'static CcConfig, GenericError>{
    Ok(&CC_CONF)
}
