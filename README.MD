# Rabbit-SSE-Integrator

[![Build Status](https://github.com/EslamYasser-Dev/Rabbit-SSE-Integrator/actions/workflows/ci.yml/badge.svg)](https://github.com/EslamYasser-Dev/Rabbit-SSE-Integrator/actions/workflows/ci.yml)
[![Docker Build](https://img.shields.io/docker/cloud/build/eslamyasser/rabbit-sse-integrator)](https://hub.docker.com/r/eslamyasser/rabbit-sse-integrator)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 📌 Overview

Rabbit-SSE-Integrator is a lightweight service written in **Rust** that integrates **RabbitMQ** with **Server-Sent Events (SSE)**.  
It allows backend services to publish messages to RabbitMQ and automatically streams them to web clients in real time using SSE.

## ✨ Features

- 🚀 High-performance Rust backend.
- 🔗 Real-time message streaming via **SSE**.
- 🐇 RabbitMQ integration for event-driven architecture.
- 🐳 Docker-ready for easy deployment.
- ⚡ Lightweight and fast.

## 📂 Project Structure

```
.
├── src/                 # Source code
├── Dockerfile           # Docker build file
├── .github/workflows/   # CI/CD pipelines
├── Cargo.toml           # Rust dependencies & metadata
└── README.md            # Documentation
```

## 🛠 Installation

### Prerequisites

- Rust (nightly recommended if using edition 2024)
- RabbitMQ instance running

### Clone and Run

```bash
git clone https://github.com/EslamYasser-Dev/Rabbit-SSE-Integrator.git
cd Rabbit-SSE-Integrator
cargo run
```

### Run with Docker

```bash
docker build -t rabbit-sse-integrator .
docker run -p 8080:8080 --env RABBITMQ_URL=amqp://guest:guest@localhost:5672 rabbit-sse-integrator
```

## ⚙️ Configuration

Environment variables:

| Variable       | Description                   | Default |
|----------------|-------------------------------|---------|
| `RABBITMQ_URL` | RabbitMQ connection string    | amqp://guest:guest@localhost:5672 |
| `PORT`         | Server port for SSE endpoint  | 8080    |

## 📡 Usage

Once running, clients can connect to the SSE endpoint:

```http
GET /events
Accept: text/event-stream
```

The service will push RabbitMQ messages directly to connected clients.

## 🤝 Contributing

Contributions are welcome!  
Please open issues and pull requests to help improve the project.

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.
