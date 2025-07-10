use lib::{
    bus,
    dto::QueueType,
};
use color_eyre::eyre::{eyre, Result};
use std::str::FromStr;


#[derive(Debug)]
pub struct Queue {
    bus: bus::Bus,
}

impl Queue {
    pub fn init() -> Result<()>{
        tracing::info!("Initializing queues");
    // for each queuetype, create a queue
        for queue in QueueType::iter() {

            let bus = bus::Bus::new();
            match bus.create_queue(
                queue.to_string(),
                Some(queue.channel_id())
            ) {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Error creating queue: {:?}", queue.to_string());
                    tracing::error!("Error: {:?}", e.to_string());
                }
            }
            tracing::info!("Queue {} initialized", queue);
        }
        tracing::info!("Queues initialized");

        Ok(())
    }

    pub fn new() -> Self {
        let bus = bus::Bus::new();
        Self { bus }
    }

    // Distribute the data to the appropriate topic in the queue
    #[tracing::instrument]
    pub(super) fn dispatch(
        &self,
        request: crate::http::Request
    ) -> color_eyre::Result<()> {
        let span = tracing::span!(tracing::Level::INFO, "dispatch");
        let _enter = span.enter();
        // Find which queue to send the data to
        let queue = self.picker(request.requestee())?;
        let message = crate::http::Message::new(request);

        self.bus.send(message, Some(queue.channel_id())).unwrap();

        Ok(())
    }

    #[tracing::instrument]
    fn picker(&self,  to: &str) -> color_eyre::Result<QueueType> {
        tracing::info!("Picking queue for {}", to);
        let queue = QueueType::from_str(&to).map_err(|_| eyre!("Invalid queue"))?;
        Ok(queue)
    }

}
