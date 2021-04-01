#[derive(Clone, Deserialize)]
pub struct MetaConf {
    pub xml_seed: XmlSeedConf,
}

#[derive(Clone, Deserialize)]
pub struct XmlSeedConf{
    pub translate_indicator: bool,
    pub translate_date_time: bool,
}

lazy_static! {
    pub static ref META_CONF: MetaConf = MetaConf::load() ;
}

impl MetaConf{
    pub fn load() -> Self {
        let cnt=include_str!("meta_conf.toml");
        let conf: MetaConf=toml::from_str(cnt).expect("meta_conf.toml");
        conf
    }
}

