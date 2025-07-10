# Homelab Internal Services 🦀

This repository is a monorepo containing a collection of bespoke Rust services that power the automation and internal APIs of my homelab. These services are designed to be lightweight, efficient, and run as containers within the Kubernetes cluster. _At least, some of them are intended to be run as containers._

## ▪️ About The Architecture

These services operate on an event-driven model using an AMQP message broker. A central API gateway receives commands, publishes them to queues, and dedicated workers consume these messages to perform specific tasks.

## ▪️ Services

This workspace contains the following key services:

| Service | Description |
| :--- | :--- |
| **`quard`** | A component that sits in front of the bridge and intercepts the incoming requests. |
| **`collector`** | Main purpose of this component is to collect data from the remote queue and send it to the local queue. |
| **`killswitch`**| This is a simple killswitch for the machines in the network. Main purpose is shutting down the local windows machine.  |
| **`echo`**| A discord bot for basic stuff, and can communicate with the message queue for private use cases. |
| **`waker`**| A component that triggers wake on lan on the local machines. |

## ▪️ Getting Started

This project is a standard Rust workspace.

1.  **Clone the repository:**
    ```bash
    git clone git@github.com:kondanta/internal-rust-services.git
    cd internal-rust-services
    ```

2.  **Build a specific service:**
    ```bash
    cargo build --package quard --release
    ```

3.  **Build all services:**
    ```bash
    cargo build --workspace --release
    ```

## ▪️ License

This project is dual-licensed under the terms of both the **[MIT License](LICENSE-MIT)** and the **[Apache License 2.0](LICENSE-APACHE)** to adhere to Rust community standards, even for internal use.