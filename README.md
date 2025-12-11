# kafka-tui

A terminal user interface for Apache Kafka.

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- Browse topics, messages, and consumer groups
- Produce messages to topics
- Create and delete topics
- Multiple connection profiles with SASL/SSL support
- Vim-style navigation (j/k)

## Installation

### From source

```bash
# Prerequisites: Rust 1.70+, CMake, OpenSSL dev headers

# Ubuntu/Debian
sudo apt install cmake libssl-dev pkg-config

# macOS
brew install cmake openssl

# Build
git clone https://github.com/yourname/kafka-tui
cd kafka-tui
cargo install --path .
```

## Quick Start

```bash
# Run
kafka-tui

# On Welcome screen:
# - Press 'n' to create new connection
# - Enter name and broker address (e.g., localhost:9092)
# - Press Enter to connect
```

## Connection Profiles

Profiles are saved to `~/.config/kafka-tui/connections.json`.

### No Authentication

```
Name: local
Brokers: localhost:9092
Auth: None
```

### SASL/PLAIN

```
Name: dev-cluster
Brokers: kafka.dev.example.com:9092
Auth: SASL/PLAIN
Username: user
Password: secret
```

### SASL/SCRAM-256 or SCRAM-512

```
Name: prod-cluster
Brokers: kafka.prod.example.com:9092
Auth: SASL/SCRAM-256
Username: admin
Password: secret
```

## Keyboard Shortcuts

### Global

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `?` / `F1` | Show help |
| `Tab` | Switch to content panel |
| `Shift+Tab` | Switch to sidebar |
| `Esc` | Go back / Close modal |
| `1` | Go to Topics |
| `2` | Go to Consumer Groups |
| `3` | Go to Brokers |

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` / `Home` | Go to top |
| `G` / `End` | Go to bottom |
| `PageUp` | Page up |
| `PageDown` | Page down |
| `Enter` | Select / Confirm |

### Topics Screen

| Key | Action |
|-----|--------|
| `Enter` / `m` | View messages |
| `n` | Create new topic |
| `/` | Filter topics |
| `Ctrl+L` | Clear filter |
| `Ctrl+R` / `F5` | Refresh |

### Messages Screen

| Key | Action |
|-----|--------|
| `v` / `Enter` | Toggle message detail |
| `p` | Produce message |
| `Ctrl+R` / `F5` | Refresh |
| `Ctrl+L` | Clear messages |

### Consumer Groups Screen

| Key | Action |
|-----|--------|
| `Enter` | View group details |
| `/` | Filter groups |
| `Ctrl+L` | Clear filter |
| `Ctrl+R` / `F5` | Refresh |

### Welcome Screen

| Key | Action |
|-----|--------|
| `Enter` | Connect to selected profile |
| `n` | New connection |
| `d` | Delete selected profile |

## Docker Compose Example

```yaml
services:
  kafka:
    image: confluentinc/cp-kafka:7.5.0
    ports:
      - "9092:9092"
    environment:
      KAFKA_NODE_ID: 1
      KAFKA_PROCESS_ROLES: broker,controller
      KAFKA_CONTROLLER_QUORUM_VOTERS: 1@kafka:9093
      KAFKA_LISTENERS: PLAINTEXT://kafka:29092,CONTROLLER://kafka:9093,PLAINTEXT_HOST://0.0.0.0:9092
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:29092,PLAINTEXT_HOST://localhost:9092
      KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT,PLAINTEXT_HOST:PLAINTEXT
      KAFKA_CONTROLLER_LISTENER_NAMES: CONTROLLER
      KAFKA_INTER_BROKER_LISTENER_NAME: PLAINTEXT
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      CLUSTER_ID: MkU3OEVBNTcwNTJENDM2Qk
```

Then connect with: `localhost:9092`, Auth: `None`

## License

MIT
