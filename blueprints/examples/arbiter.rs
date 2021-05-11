use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct Ping;

#[derive(Default)]
struct MyActor1;

impl Actor for MyActor1 {
    type Context = Context<Self>;
}
impl actix::Supervised for MyActor1 {}

impl ArbiterService for MyActor1 {
   fn service_started(&mut self, ctx: &mut Context<Self>) {
      println!("Service started");
   }
}

impl Handler<Ping> for MyActor1 {
   type Result = ();

   fn handle(&mut self, _: Ping, ctx: &mut Context<Self>) {
      println!("ping");
   }
}

struct MyActor2;

impl Actor for MyActor2 {
   type Context = Context<Self>;

   fn started(&mut self, _: &mut Context<Self>) {
      // get MyActor1 address from the registry
      let act = MyActor1::from_registry();
      act.do_send(Ping);
   }
}

fn main() {
    // initialize system
    let code = System::new().block_on(async {
        // Start MyActor2 in new Arbiter
        Arbiter::new().spawn_fn(|| {
            MyActor2.start();
        });
    });
}

