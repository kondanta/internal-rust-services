// Message bus for the collector. It'll read from quard and write to the queue.
// More specifically, from a public queue to a private queue.

use std::fmt::Debug;
use std::borrow::Cow;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions};

use color_eyre::eyre::{eyre, Result};
use eyre::bail;

use crate::dto::BusRequest;

#[derive(Debug)]
pub struct Config {
    pub amqp_url: String,
}

// Main struct for the bus
// Listens to the message queue and sends the data to another queue
#[derive(Debug)]
pub struct Bus {
    config: Config,
}

impl Bus {
    pub fn new() -> Self {
        let amqp_url = std::env::var("AMQP_URL")
            .unwrap_or("amqp://guest:guest@localhost:5672".to_string());

        let config = Config {
            amqp_url,
        };

        Self { config }
    }

    #[tracing::instrument]
    pub fn create_queue(
        &self,
        queue_name: String,
        channel_id: Option<u16>,
    ) -> Result<()> {
        tracing::info!("Creating queue {}", queue_name);
        let mut connection = match Connection::insecure_open(&self.config.amqp_url) {
            Ok(c) => c,
            Err(e) => bail!(e),
        };

        let channel = connection.open_channel(channel_id).unwrap();

        let _ = channel.queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
        ).map_err(|e| {
            tracing::error!("Error declaring queue: {:?}", e);
            eyre!(e)
        })?;

        connection.close().map_err(|e| eyre!(e))?;
        tracing::info!("Queue created");

        Ok(())
    }

    pub fn listen(
        &self,
        queue_name: String,
        channel_id: Option<u16>,
        f : fn(body: Cow<str>),
    ) -> Result<()> {
        let mut connection = Connection::insecure_open(&self.config.amqp_url)?;

        let channel = connection.open_channel(channel_id)?;

        let queue = channel.queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
        )?;

        let consumer = queue.consume(ConsumerOptions::default())?;

        for(i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
                    tracing::info!("({:>3}) Received [{}]", i, body);
                    f(body.clone());
                    if body == "quit" {
                        tracing::info!("Quitting");
                        break;
                    }
                    consumer.ack(delivery)?;
                }
                other => {
                    tracing::info!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }

        connection.close().map_err(|e| eyre!(e))?;

        Ok(())
    }

    // todo: implement data struct.
    #[tracing::instrument]
    pub fn send<T: BusRequest + Debug>(
        &self,
        data: T,
        channel_id: Option<u16>,
    ) -> Result<()> {
        let mut connection = Connection::insecure_open(&self.config.amqp_url).unwrap();

        let channel = connection.open_channel(channel_id).unwrap();

        let exchange = Exchange::direct(&channel);

        let message = json_bytes(data.payload().to_string())?;
        exchange.publish(
            Publish::new(
                &message,
                data
                .requestee()
                .to_string()
                .to_lowercase()
            )
        ).unwrap();

        connection.close().map_err(|e| eyre!(e)).unwrap();

        Ok(())
    }
}

fn  json_bytes(data: String) -> Result<Vec<u8>>  {
    serde_json::to_vec(&data).map_err(|e| eyre!(e))
}