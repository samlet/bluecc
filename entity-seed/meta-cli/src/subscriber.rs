use redis::{PubSubCommands, ControlFlow, RedisResult};

pub fn observe(topics: &Vec<String>) -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut count = 0;

    info!(".. observe on {:?}", topics);
    let topics:Vec<Vec<u8>>=topics.iter()
        .map(|e|e.to_owned().into_bytes())
        .collect();
    con.subscribe(&*topics, |msg| {
        // assert_eq!(msg.get_channel(), Ok(String::from("foo")));
        let channel:String=msg.get_channel().unwrap();
        let payload:String=msg.get_payload().unwrap();
        println!("{}: {}", channel, payload);

        // increment messages seen counter
        count += 1;
        match count {
            // stop after receiving 10000 messages
            10000 => ControlFlow::Break(()),
            _ => ControlFlow::Continue,
        }
    })
}

