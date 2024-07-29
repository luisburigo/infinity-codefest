use amiquip::{AmqpProperties, Channel, Connection, ExchangeDeclareOptions, ExchangeType, Publish};
use crate::types::transaction::event::{CreatedTransactionEventPayload, PendingTransactionEventPayload};
use crate::types::transaction::types::{Transaction, TransactionStatus};
use crate::types::user::event::{CreatedUserEventPayload, PendingUserEventPayload};
use crate::types::user::types::{User, UserStatus};

pub struct TransactionProducer {
    channel: Channel,
}

impl TransactionProducer {
    pub fn new(connection: &mut Connection) -> Self {
        let channel = connection.open_channel(None).unwrap();
        TransactionProducer { channel }
    }

    pub fn publish(&self, tx: Transaction) -> Result<(), amiquip::Error> {
        let body = match tx.clone().status.unwrap() {
            TransactionStatus::Review => {
                let payload = PendingTransactionEventPayload::from(tx);
                serde_json::to_string(&payload).expect("Failed to serialize event")
            },
            TransactionStatus::Success => {
                let payload = CreatedTransactionEventPayload::from(tx);
                serde_json::to_string(&payload).expect("Failed to serialize event")
            },
            _ => return Ok(()),
        };

        let exchange = self.channel.exchange_declare(
            ExchangeType::Fanout,
            "transactions".to_owned(),
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