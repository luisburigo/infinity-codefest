use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable};
use crate::types::user::event::UserEventMessage;


/// # Examples
///
/// ```rust
///     let connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
///
///     let user_consumer = UserConsumer::start(connection)
///             .expect("Failed to start UserConsumer");
///     user_consumer.subscribe(|event| {
///         match event {
///             UserEventMessage::Requested(payload) => {
///                 let user = payload.to_user();
///                 println!("User: {:?}", user);
///             },
///             event => {
///                 eprintln!("Unknown event: {:?}", event);
///             }
///         }
///     })
/// ```
pub struct UserConsumer {
    connection: Connection,
    channel: amiquip::Channel,
}

impl UserConsumer {
    pub fn start(mut connection: Connection) -> Result<Self, amiquip::Error> {
        let channel = connection.open_channel(None)?;
        Ok(UserConsumer { connection, channel })
    }

    pub fn subscribe<F>(&self, callback: F) -> Result<(), amiquip::Error>
    where
        F: Fn(UserEventMessage) + 'static,
    {
        let queue = self.channel.queue_declare("", Default::default())?;
        self.channel.queue_bind(queue.name(), "users", "", FieldTable::new())?;
        let consumer = queue.consume(ConsumerOptions::default())?;

        for message in consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body).to_string();

                    let message = UserEventMessage::from(body);
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

    pub fn close(self) -> Result<(), amiquip::Error> {
        self.connection.close()
    }
}