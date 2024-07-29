use amiquip::{AmqpProperties, Channel, Connection, ExchangeDeclareOptions, ExchangeType, Publish};
use crate::types::user::event::{CreatedUserEventPayload, PendingUserEventPayload};
use crate::types::user::types::{ToUser, User, UserStatus};

pub struct UserProducer {
    channel: Channel,
}

impl UserProducer {
    pub fn new(connection: &mut Connection) -> Self {
        let channel = connection.open_channel(None).unwrap();
        UserProducer { channel }
    }

    pub fn publish(&self, user: User) -> Result<(), amiquip::Error> {
        let body = match user.clone().status.unwrap() {
            UserStatus::Review => {
                let payload = PendingUserEventPayload::from(user);
                serde_json::to_string(&payload).expect("Failed to serialize event")
            },
            UserStatus::Success => {
                let payload = CreatedUserEventPayload::from(user);
                serde_json::to_string(&payload).expect("Failed to serialize event")
            },
            _ => return Ok(()),
        };

        let exchange = self.channel.exchange_declare(
            ExchangeType::Fanout,
            "users".to_owned(),
            ExchangeDeclareOptions::default(),
        )?;
        self.channel.basic_publish(exchange.name(), Publish {
            body: body.as_bytes(),
            routing_key: "".to_string(),
            mandatory: false,
            immediate: false,
            properties: AmqpProperties::default().with_app_id("NG".to_owned()),
        })
    }
}