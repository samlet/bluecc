extern crate capnp_rpc;

pub mod hello_world_capnp {
  include!(concat!(env!("OUT_DIR"), "/hello_world_capnp.rs"));
}

pub mod client;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return client::main().await,
            "server" => return server::main().await,
            _ => ()
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
    Ok(())
}
