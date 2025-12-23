# kafka-tui

A terminal user interface for Apache Kafka.

![Rust](https://img.shields.io/badge/rust-1.83%2B-orange)
![Edition](https://img.shields.io/badge/edition-2021-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Topics**: Browse, create, delete, purge topics
- **Messages**: View, produce, consume messages in real-time
- **Consumer Groups**: Monitor groups, members, offsets and lag
- **Brokers**: View cluster broker information
- **Partitions**: Add partitions, view partition details
- **Configuration**: View and modify topic configurations
- **Multiple Connections**: Save connection profiles with SASL/SSL support
- **Vim-style Navigation**: j/k, g/G, Ctrl+D/U
- **Filtering**: Search topics and consumer groups
- **Logs**: Built-in application log viewer

## Installation

### Prerequisites

- Rust 1.83+
- CMake
- OpenSSL development headers
- libsasl2 (for SASL authentication)

### Ubuntu/Debian

```bash
sudo apt install cmake libssl-dev libsasl2-dev pkg-config
```

### Fedora/RHEL

```bash
sudo dnf install cmake openssl-devel cyrus-sasl-devel
```

### macOS

```bash
brew install cmake openssl
```

### Build from source

```bash
git clone https://github.com/yourname/kafka-tui
cd kafka-tui
cargo install --path .
```

## Quick Start

```bash
kafka-tui
```

On the Welcome screen:
1. Press `n` to create a new connection
2. Enter a name and broker address (e.g., `localhost:9092`)
3. Select authentication type if needed
4. Press `Enter` to connect

## Screenshots

```
┌─ Welcome to Kafka TUI ───────────────────────────────────────┐
│   _  __      __ _           _____ _   _ ___                  │
│  | |/ /__ _ / _| | ____ _  |_   _| | | |_ _|                 │
│  | ' // _` | |_| |/ / _` |   | | | | | || |                  │
│  | . \ (_| |  _|   < (_| |   | | | |_| || |                  │
│  |_|\_\__,_|_| |_|\_\__,_|   |_|  \___/|___|                 │
│                                                              │
│  ┌─ Saved Connections ─────────────────────────────────────┐ │
│  │  local (localhost:9092)                                 │ │
│  │  dev-cluster (kafka.dev.example.com:9092)               │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                              │
│  [Enter] Connect  [n] New connection  [q] Quit               │
└──────────────────────────────────────────────────────────────┘
```

## Connection Profiles

Profiles are stored in `~/.local/share/kafka-tui/kafka-tui.db` (SQLite).

### Supported Authentication

| Type | Description |
|------|-------------|
| None | No authentication |
| SASL/PLAIN | Username/password (plaintext) |
| SASL/SCRAM-256 | SCRAM-SHA-256 authentication |
| SASL/SCRAM-512 | SCRAM-SHA-512 authentication |

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
| `4` | Go to Logs |

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` / `Home` | Go to top |
| `G` / `End` | Go to bottom |
| `Ctrl+D` / `PageDown` | Page down |
| `Ctrl+U` / `PageUp` | Page up |
| `Enter` | Select / Confirm |

### Topics Screen

| Key | Action |
|-----|--------|
| `Enter` / `m` | View messages |
| `i` | View topic details |
| `n` | Create new topic |
| `d` | Delete topic |
| `/` | Filter topics |
| `Ctrl+L` | Clear filter |
| `Ctrl+R` / `F5` | Refresh |

### Topic Details Screen

| Key | Action |
|-----|--------|
| `Tab` | Switch between Partitions/Config tabs |
| `a` | Add partitions |
| `e` | Edit configuration |
| `x` | Purge messages |

### Messages Screen

| Key | Action |
|-----|--------|
| `v` / `Enter` | Toggle message detail |
| `p` | Produce message |
| `c` | Start/stop consuming |
| `Ctrl+R` / `F5` | Refresh |
| `Ctrl+L` | Clear messages |

### Consumer Groups Screen

| Key | Action |
|-----|--------|
| `Enter` | View group details |
| `Tab` | Switch between Members/Offsets tabs |
| `/` | Filter groups |
| `Ctrl+L` | Clear filter |
| `Ctrl+R` / `F5` | Refresh |

### Logs Screen

| Key | Action |
|-----|--------|
| `f` | Cycle log level filter |
| `Ctrl+L` | Clear logs |

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

Connect with: `localhost:9092`, Auth: `None`

## Architecture

```
src/
├── app/
│   ├── handlers/     # Domain-specific action handlers
│   ├── actions.rs    # Action and Command enums
│   ├── runner.rs     # Main event loop
│   ├── state.rs      # Application state
│   ├── update.rs     # State update coordinator
│   └── validation.rs # Form input validation
├── events/           # Keyboard event handling
├── kafka/
│   ├── client.rs     # Kafka client wrapper
│   ├── admin_ffi.rs  # Low-level FFI operations
│   └── config.rs     # Connection configuration
├── storage/          # SQLite connection storage
└── ui/
    ├── screens/      # Screen renderers
    ├── components/   # Reusable UI components
    └── theme.rs      # Color theme
```

## License

MIT
