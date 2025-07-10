use std::str::FromStr;
use std::fmt;

use serde::{Deserialize, Serialize};

pub trait BusRequest {
    fn requestee(&self) -> QueueType;
    fn requestor(&self) -> &str;
    fn payload(&self) -> &str;
    fn to_json(&self) -> serde_json::Value;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QueueType {
    Discord,
    Drink,
    Game,
    Killswitch,
    Waker,
    Work,
}

pub struct QueueTypeIterator {
    index: usize,
}

impl Iterator for QueueTypeIterator {
    type Item = QueueType;

    fn next(&mut self) -> Option<Self::Item> {
        let variants = [
            QueueType::Discord,
            QueueType::Drink,
            QueueType::Game,
            QueueType::Killswitch,
            QueueType::Waker,
            QueueType::Work,
        ];

        if self.index < variants.len() {
            let variant = variants[self.index].clone();
            self.index += 1;
            Some(variant)
        } else {
            None
        }
    }
}

impl QueueType {
    pub fn iter() -> QueueTypeIterator {
        QueueTypeIterator { index: 0 }
    }
}

impl FromStr for QueueType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let s = s.to_lowercase();
        match s.as_str() {
            "discord" => Ok(QueueType::Discord),
            "drink" => Ok(QueueType::Drink),
            "game" => Ok(QueueType::Game),
            "killswitch" => Ok(QueueType::Killswitch),
            "waker" => Ok(QueueType::Waker),
            "work" => Ok(QueueType::Work),
            _ => Err(())
        }
    }
}

impl fmt::Display for QueueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueueType::Discord => write!(f, "discord"),
            QueueType::Drink => write!(f, "drink"),
            QueueType::Game => write!(f, "game"),
            QueueType::Killswitch => write!(f, "killswitch"),
            QueueType::Waker => write!(f, "waker"),
            QueueType::Work => write!(f, "work"),
        }
    }
}

impl QueueType {
    pub fn channel_id(&self) -> u16 {
        match self {
            QueueType::Discord => 10,
            QueueType::Drink => 11,
            QueueType::Game => 12,
            QueueType::Killswitch => 13,
            QueueType::Waker => 14,
            QueueType::Work => 15,
        }
    }
}
