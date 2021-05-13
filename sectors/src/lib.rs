#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

pub mod pubsub;
mod subsys;

#[macro_use] extern crate capnp_rpc;

pub mod pubsub_capnp {
  include!(concat!(env!("OUT_DIR"), "/protocols/pubsub_capnp.rs"));
}
pub mod addressbook_capnp {
  include!(concat!(env!("OUT_DIR"), "/protocols/addressbook_capnp.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
