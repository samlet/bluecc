use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DirectorConf{
    pub default_queue: String,
    pub broker_uri: String,
}

pub struct SrvDirector{
    conf: DirectorConf,
}

impl SrvDirector{
    pub fn new() -> Self {
        SrvDirector { conf: DirectorConf{
            default_queue:"ink_queue".to_string(),
            broker_uri: "amqp://127.0.0.1:5672".to_string(),
        }}
    }

    pub async fn srv(&self) -> crate::Result<()>{
        let conn = Connection::connect(self.conf.broker_uri.as_str(),
                                       ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        channel.queue_declare(self.conf.default_queue.as_str(),
                              QueueDeclareOptions{durable:true, ..QueueDeclareOptions::default()},
                              FieldTable::default()) .await?;

        let consumer = channel
            .basic_consume(
                self.conf.default_queue.as_str(),
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!(" [*] Waiting for messages. To exit press CTRL+C");

        for delivery in consumer {
            let (channel, delivery) = delivery?;
            let data=std::str::from_utf8(&delivery.data)?;
            self.proc(data).await?;
            channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await?;
        }

        Ok(())
    }

    async fn proc(&self, data:&str) -> crate::Result<()>{
        println!(" [x] Received {:?}", data);
        println!(" [x] Done");

        Ok(())
    }
}

