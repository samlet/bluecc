use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct Ping;

#[derive(Default)]
struct MyActor1;

impl Actor for MyActor1 {
    type Context = Context<Self>;
}

impl Supervised for MyActor1 {}

// System registry serves same purpose as Registry, except it is shared across all arbiters.
impl SystemService for MyActor1 {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Service started");
    }
}

impl Handler<Ping> for MyActor1 {
    type Result = ();

    fn handle(&mut self, _: Ping, _ctx: &mut Context<Self>) {
        println!("ping");
    }
}

struct MyActor2;

impl Actor for MyActor2 {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        let act = MyActor1::from_registry();
        act.do_send(Ping);
    }
}

#[actix::main]
async fn main() {
    println!("🎩 creating client client");
    let _client = MyActor2.start();

    println!("🎩 Ctrl-C received, shutting down");
    tokio::signal::ctrl_c().await.unwrap();
    System::current().stop();
}
