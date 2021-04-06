use std::time;
use web3::{
    contract::{Contract, Options},
    types::U256,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = env_logger::try_init();
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    // Get current balance
    let balance = web3.eth().balance(accounts[0], None).await?;

    println!("Balance: {}", balance);

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = std::fs::read_to_string("fixtures/dist/SimpleStorage.bin")?;
    let abi=std::fs::read("fixtures/dist/SimpleStorage.abi")?;
    // Deploying a contract
    // let duration=10;
    let duration=5;
    println!("poll interval {} secs", duration);
    let contract = Contract::deploy(web3.eth(), &*abi)?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(duration))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode.as_str(), (), accounts[0])
        .await?;

    println!("Deployed at: {}", contract.address());

    // interact with the contract
    let result = contract.query("get", (),
                                None, Options::default(), None);
    let storage: U256 = result.await?;
    println!("Get Storage: {}", storage);

    // Change state of the contract
    let tx = contract.call("set", (42_u32,), accounts[0],
                           Options::default()).await?;
    println!("TxHash: {}", tx);

    let duration=5;
    println!("wait result for {} secs", duration);
    // consider using `async_std::task::sleep` instead.
    std::thread::sleep(std::time::Duration::from_secs(duration));

    // View changes made
    let result = contract.query("get", (),
                                None, Options::default(), None);
    let storage: U256 = result.await?;
    println!("Get again: {}", storage);

    Ok(())
}
