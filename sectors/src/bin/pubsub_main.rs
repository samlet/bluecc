use sectors::pubsub::{run_client, run_server};

/*
# Start the server like this:
$ cargo run --bin pubsub_main server 127.0.0.1:4000
# Then start any number of clients like this:
$ cargo run --bin pubsub_main client 127.0.0.1:4000
 */
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return run_client().await,
            "server" => return run_server().await,
            _ => ()
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
    Ok(())
}
