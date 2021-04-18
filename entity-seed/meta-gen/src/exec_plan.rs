use std::path::PathBuf;
use std::collections::HashMap;
use crate::{process_seed, ServiceMeta};

type CheckStatus=(HashMap<String, (bool,bool)>, HashMap<String, (bool,bool)>);
fn check_srv_pks(srvs: &mut ServiceMeta, srv_name: &str) -> crate::Result<CheckStatus> {
    let srv = srvs.srv(srv_name)?;
    let ent_name = srv.get_entity_name();
    let mut pk_incs = HashMap::new();
    let mut resp_incs = HashMap::new();
    if !ent_name.is_empty() {
        let ent = srvs.get_entity_model(ent_name.as_str())?;
        let pks = ent.pks();

        // check input parameters
        let in_pars = srvs.srv_input_params(srv_name)?;
        for par in in_pars {
            let pname = par.name;
            if pks.contains(&pname) {
                pk_incs.insert(pname, (true, par.optional));
            }
        }

        // check output parameters
        let out_pars = srvs.srv_output_params(srv_name)?;
        for par in out_pars {
            let pname = par.name;
            if pks.contains(&pname) {
                resp_incs.insert(pname, (true, par.optional));
            }
        }
    }

    Ok((pk_incs, resp_incs))
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use deles::delegators::pretty;

    #[test]
    fn check_pks_works() -> anyhow::Result<()> {
        let mut srvs = ServiceMeta::load()?;
        let srv_name="createPerson";
        let pk_incs=check_srv_pks(&mut srvs, srv_name)?;
        println!("{:?}", pk_incs);
        println!("{:?}", check_srv_pks(&mut srvs, "createInvoice")?);
        println!("{:?}", check_srv_pks(&mut srvs, "createCustRequest")?);
        println!("{:?}", check_srv_pks(&mut srvs, "createCustRequestItem")?);
        println!("{:?}", check_srv_pks(&mut srvs, "createProductStore")?);
        println!("{:?}", check_srv_pks(&mut srvs, "createQuote")?);
        Ok(())
    }

    #[test]
    fn seed_induce_works() -> crate::Result<()> {
        use std::fs;
        let path = PathBuf::from("test_files/ExampleDemoData.xml");
        let cnt = fs::read_to_string(path.as_path())?;
        let rs = process_seed(cnt.as_str())?;
        // println!("{}", pretty(&rs));

        let mut srvs = ServiceMeta::load()?;
        for seed in rs {
            let ent_name=seed.entity;
            let result = srvs.get_related_srvs(ent_name.as_str())?;
            let create_srv=result.iter().find(|s|s.starts_with("create"));
            if let Some(srv)=create_srv{
                println!("seed {} with creator {}", ent_name, srv);
                // 如果是服务形式, 则需要处理主键由服务产生的情况
            }else{
                println!("seed {} with delegator storeOrUpdate", ent_name);
                // 这种情形下需要数据包含主键
            }
        }

        Ok(())
    }
}
