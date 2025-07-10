use lib::dto::QueueType;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    body: String,
    requestee: String,
    requestor: String,
}

impl Request {
    pub fn requestee(&self) -> &str {
        &self.requestee
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    request: Request,
}

impl Message {
    pub fn new(request: Request) -> Self {
        Self { request }
    }
}

impl lib::dto::BusRequest for Message {
    fn requestee(&self) -> QueueType {
        self.request.requestee.parse().expect("Invalid queue type")
    }

    fn requestor(&self) -> &str {
        &self.request.requestor
    }

    fn payload(&self) -> &str {
        &self.request.body
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "requestee": self.request.requestee,
            "requestor": self.request.requestor,
            "body": self.request.body,
        })
    }
}