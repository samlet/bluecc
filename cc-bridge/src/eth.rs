use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ContractFixture{
    pub abi: ethabi::Contract,
    #[serde(default)]
    pub all_source_paths: HashMap<String, String>,
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use itertools::Itertools;
    use common::prelude::pretty;

    #[tokio::test]
    async fn accounts_works() -> web3::Result<()> {
        let transport = web3::transports::Http::new("http://localhost:8545")?;
        let web3 = web3::Web3::new(transport);

        println!("Calling accounts.");
        let mut accounts = web3.eth().accounts().await?;
        println!("Accounts: {:?}", accounts);
        accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

        println!("Calling balance.");
        for account in accounts {
            let balance = web3.eth().balance(account, None).await?;
            println!("Balance of {:?}: {}", account, balance);
        }

        Ok(())
    }

    #[test]
    fn abi_json_works() -> anyhow::Result<()> {
        // let bytes=std::fs::read("fixtures/dist/SimpleEvent.json")?;
        let bytes=std::fs::read("fixtures/dist/token.json")?;
        let abi = ethabi::Contract::load(&*bytes)?;
        let functions=abi.functions.iter()
            .map(|(k,_)|k).collect_vec();
        println!("{}", pretty(&functions));

        Ok(())
    }

    #[test]
    fn contract_json_works() -> anyhow::Result<()> {
        let bytes=std::fs::read("fixtures/dist/SimpleEvent.json")?;
        let fixture:ContractFixture=serde_json::from_reader(&*bytes)?;
        let abi = fixture.abi;
        let functions=abi.functions.iter()
            .map(|(k,_)|k).collect_vec();
        println!("{}", pretty(&functions));

        Ok(())
    }
}


