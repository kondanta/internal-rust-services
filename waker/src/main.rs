use std::env;

use lib::{bus, dto::QueueType};

mod wol;
mod util;


fn main() -> std::io::Result<()> {
    let bus = bus::Bus::new();
    let queue = QueueType::Waker;
    bus.listen(
        queue.to_string(),
        Some(queue.channel_id()),
        waker
    ).unwrap();

    Ok(())
}

fn waker(body: std::borrow::Cow<str>) {
    if body.contains("waker") {
        wol();
    }
}

// WoL
fn wol() {
    println!("Waking up the device");
    let mac = env::var("MAC_ADDRESS").unwrap_or_else(|_| {
        panic!("MAC_ADDRESS environment variable is not set");
    });
    wol::create_wol_message(mac).ok();
}