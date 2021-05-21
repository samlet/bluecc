use actix::prelude::*;

/// Define `Ping` message
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// Actor
struct MyActor {
    count: usize,
}

/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

// #[actix::main]
// #[tokio::main(flavor = "current_thread")]
// async fn main() -> anyhow::Result<()>{
fn main() -> anyhow::Result<()>{
    // +
    // let local = tokio::task::LocalSet::new();
    // let sys = actix_rt::System::run_in_tokio("server", &local);
    // +

    // 如果用#[actix::main]标注, 则不需要block_on块
    // https://github.com/tokio-rs/tokio/issues/2095
    System::new().block_on(async {
        // start new actor
        let addr = MyActor { count: 10 }.start();
        // send message and get future for result
        let res = addr.send(Ping(10)).await;
        // handle() returns tokio handle
        println!("RESULT: {}", res.unwrap() == 20);
    });

    // sys.await?; // +

    // stop system and exit
    System::current().stop();

    Ok(())
}
