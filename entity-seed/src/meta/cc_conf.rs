use crate::{GenericError, exists};

#[derive(Clone, Deserialize)]
pub struct CcConfig {
    pub ofbiz_loc: String,
    pub srv_root: String,
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

    pub fn get_ofbiz_root(&self) -> String{
        use envmnt::{ExpandOptions, ExpansionType};
        let mut options = ExpandOptions::new();
        options.expansion_type = Some(ExpansionType::Unix);
        envmnt::expand(self.ofbiz_loc.as_str(), Some(options))
    }

    pub fn get_srv_root(&self) -> String {
        format!("{}/{}", self.get_ofbiz_root(), self.srv_root)
    }

    pub fn get_component_conf_path(&self) -> String{
        format!("{}/ofbiz-component.xml", self.get_srv_root())
    }
}

lazy_static! {
    pub static ref CC_CONF: CcConfig = CcConfig::load() ;
}

pub fn cc_conf() -> Result<&'static CcConfig, GenericError>{
    Ok(&CC_CONF)
}
