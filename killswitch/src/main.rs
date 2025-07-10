use std::process::Command;
use lib::{bus, dto::QueueType};

fn main() {
    let bus = bus::Bus::new();
    let queue = QueueType::Killswitch;
    bus.listen(
        queue.to_string(),
        Some(queue.channel_id()),
        killer
    ).unwrap();
}

fn killer(body: std::borrow::Cow<str>) {
    if body.contains("shutdown") {
        shutdown();
    }
}

fn shutdown() {
    // Create a command to execute shutdown command
    let command = Command::new("shutdown")
        .arg("/s")
        .arg("/t")
        .arg("0")
        .output();

    match command {
        Ok(_) => println!("Shutdown initiated"),
        Err(e) => println!("Error initiating shutdown: {:?}", e),
    }
}
