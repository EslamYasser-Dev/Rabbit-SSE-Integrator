# Rabbit-SSE-Integrator

[![Build Status](https://github.com/EslamYasser-Dev/Rabbit-SSE-Integrator/actions/workflows/ci.yml/badge.svg)](https://github.com/EslamYasser-Dev/Rabbit-SSE-Integrator/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/EslamYasser-Dev/Rabbit-SSE-Integrator)](LICENSE)

> High-performance connector to stream messages from RabbitMQ to clients using Server-Sent Events (SSE).

---

##  Overview

**Rabbit-SSE-Integrator** enables real-time web applications to listen to RabbitMQ message queues via SSE. It transforms messages from RabbitMQ into lightweight, continuous HTTP streams—ideal for live dashboards, event-driven UIs, or low-latency notification systems.

---

##  Features

- **Blazing speed** — Built for high-throughput message delivery.
- **Simple setup** — Plug-and-play SSE endpoint for any RabbitMQ exchange/queue.
- **Scalable** — Handles many concurrent SSE clients efficiently.
- **Flexible routing** — Map multiple queues or routing keys to separate endpoints.
- **Reliable** — Supports reconnecting SSE clients and RabbitMQ error handling.

---

##  Prerequisites

- Rust (≥ 1.XX — adapt per your setup)
- RabbitMQ server (accessible with network permissions)
- Optional: Docker (for containerized deployments)

---

##  Quick Start

### Build and Run Locally

```bash
git clone https://github.com/EslamYasser-Dev/Rabbit-SSE-Integrator.git
cd Rabbit-SSE-Integrator
cargo build --release

# Run with default settings — refer to configuration section for options
./target/release/rabbit-sse-integrator
