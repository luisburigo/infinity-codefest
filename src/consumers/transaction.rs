use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable};
use crate::types::transaction::event::TransactionEventMessage;

pub struct TransactionConsumer {
    channel: amiquip::Channel,
}

impl TransactionConsumer {
    pub fn start(mut connection: &mut Connection) -> Result<Self, amiquip::Error> {
        let channel = connection.open_channel(None)?;
        Ok(TransactionConsumer { channel })
    }

    pub fn subscribe<F>(&self, callback: F) -> Result<(), amiquip::Error>
    where
        F: Fn(TransactionEventMessage) + 'static,
    {
        let queue = self.channel.queue_declare("", Default::default())?;
        self.channel.queue_bind(queue.name(), "transactions", "", FieldTable::new())?;
        let consumer = queue.consume(ConsumerOptions::default())?;

        for message in consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body).to_string();

                    let message = TransactionEventMessage::from(body);
                    match message {
                        Ok(event) => {
                            match delivery.properties.app_id() {
                                None => { callback(event) },
                                Some(app_id) => {
                                    if !app_id.eq("NG") {
                                        callback(event)
                                    }
                                }
                            }
                            delivery.ack(&self.channel)?;
                        },
                        Err(err) => {
                            eprintln!("Failed to parse message: {:?}", err);
                        }
                    }
                },
                other => {
                    eprintln!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }

        Ok(())
    }

    // pub fn close(self) -> Result<(), amiquip::Error> {
    //     self.connection.close()
    // }
}