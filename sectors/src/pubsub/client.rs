use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use crate::pubsub_capnp::{publisher, subscriber};

use capnp::capability::Promise;
use futures::{AsyncReadExt};

struct SubscriberImpl;

impl subscriber::Server<::capnp::text::Owned> for SubscriberImpl {
    fn push_message(&mut self,
                    params: subscriber::PushMessageParams<::capnp::text::Owned>,
                    _results: subscriber::PushMessageResults<::capnp::text::Owned>)
        -> Promise<(), ::capnp::Error>
    {
        println!("message from publisher: {}", pry!(pry!(params.get()).get_message()));
        Promise::ok(())
    }
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    use std::net::ToSocketAddrs;
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} client HOST:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2].to_socket_addrs().unwrap().next().expect("could not parse address");

    tokio::task::LocalSet::new().run_until(async move {
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network =
            Box::new(twoparty::VatNetwork::new(reader, writer,
                                               rpc_twoparty_capnp::Side::Client,
                                               Default::default()));
        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let publisher: publisher::Client<::capnp::text::Owned> =
            rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
        let sub = capnp_rpc::new_client(SubscriberImpl);

        let mut request = publisher.subscribe_request();
        request.get().set_subscriber(sub);

        // Need to make sure not to drop the returned subscription object.
        futures::future::try_join(rpc_system, request.send().promise).await?;
        Ok(())
    }).await
}
