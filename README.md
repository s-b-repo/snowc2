


# 📚 How Bots Should Connect and Communicate with the Modular Rust C2 Server

---

## 📌 Overview

Bots are **simple TCP clients** that connect to the C2 server’s **bot listener** (`0.0.0.0:9000` by default).  
They must maintain a **persistent** connection to:
- Receive commands.
- Respond to commands.
- Send regular **heartbeat signals** to stay marked as *alive*.

The C2 server expects very specific simple **text-based communication** over TCP.

---

## 🛠️ Bot TCP Connection

- **Target Address:**  
  `tcp://<server_ip>:9000`
  
- **Connection:**  
  Open a **plain TCP connection** (no TLS for now).
  
- **Keep it alive**:  
  Do **not** disconnect unless necessary.

---

## 📜 Message Format

- **Text-based protocol** (`\n` newline-delimited).
- Each message sent/received must end with `\n`.

### Examples:
| Direction | Message | Meaning |
|:---------:|:--------|:--------|
| Server ➔ Bot | `PING\n` | Bot should respond `PONG\n` |
| Bot ➔ Server | `HEARTBEAT\n` | Bot is still alive |
| Bot ➔ Server | `PONG\n` | Bot acknowledged server's ping |

---

## 🧩 Bot Responsibilities

Once connected, a bot must:

| Action | Details |
|:-------|:--------|
| 📥 Receive commands | Listen for incoming messages from server |
| 🏓 Handle `PING` | When server sends `PING`, reply `PONG` immediately |
| ❤️ Send heartbeats | Every 10 seconds, send `HEARTBEAT\n` without waiting for server |
| 🛑 Detect disconnection | If server closes connection, gracefully stop |

---

## 🔄 Bot Main Loop (High Level)

Pseudocode structure for the bot:

```text
- Connect to server at IP:9000
- Loop forever:
  - select:
    - if server sends a command:
      - read line
      - match command:
        - "PING" -> send "PONG"
        - (custom commands) -> act accordingly
    - every 10 seconds:
      - send "HEARTBEAT"
```

---

## 📦 Rust Bot Example (Complete Code)

Here’s a fully working **Rust Bot** example:

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{sleep, Duration};
use tokio::select;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000").await?;
    println!("Connected to server!");

    let (r, mut w) = stream.split();
    let mut reader = BufReader::new(r);
    let mut line = String::new();

    loop {
        line.clear();

        select! {
            bytes = reader.read_line(&mut line) => {
                let bytes = bytes?;
                if bytes == 0 {
                    println!("Server closed connection.");
                    break;
                }

                let command = line.trim();
                println!("Received command: {}", command);

                if command == "PING" {
                    w.write_all(b"PONG\n").await?;
                    w.flush().await?;
                    println!("Sent PONG!");
                }
                // Extend here to handle more commands
            }
            _ = sleep(Duration::from_secs(10)) => {
                // Send heartbeat every 10 seconds
                w.write_all(b"HEARTBEAT\n").await?;
                w.flush().await?;
                println!("Sent HEARTBEAT");
            }
        }
    }

    Ok(())
}
```

---

## 📊 How C2 Server Tracks Bots

When a bot connects:
- Server **generates UUID**.
- Server **stores** the bot in memory.
- Server **watches for HEARTBEAT** messages.
- If bot **disconnects** or **fails heartbeat**, server **removes** it automatically.

---

## ⚠️ Important Bot Behavior Rules

| Rule | Description |
|:-----|:------------|
| 1. Always respond to `PING` with `PONG` immediately |
| 2. Always send `HEARTBEAT` every 10 seconds |
| 3. Always newline-terminate (`\n`) every message |
| 4. Gracefully detect disconnection if server closes socket |
| 5. (Optional) Future bots can handle custom commands (e.g., download file, run shell command) |

---

# 📜 Quick Recap — Minimum Requirements for a Bot

✅ Connect TCP to `server_ip:9000`  
✅ Read and handle incoming text commands  
✅ Send `HEARTBEAT\n` every 10 seconds  
✅ Reply `PONG\n` when receiving `PING\n`  
✅ Stay connected persistently  
✅ Detect disconnection properly  

---

# 📢 Bonus Future Bot Features (Optional)

| Idea | Description |
|:-----|:------------|
| Encrypted connection | Use TLS sockets for security |
| Command execution | Bot can run OS commands sent by server |
| File upload/download | Bot can receive or send files |
| Persistence | Bot restarts automatically if killed |
| Covert heartbeats | Bot hides heartbeat in legit-looking traffic |

---

# 🛠 Tools/Crates Used

- [`tokio`](https://docs.rs/tokio/latest/tokio/) (for async TCP)
- [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) (for easy error handling)
- [`serde`](https://docs.rs/serde/latest/serde/) (for future extensions)

---

# 🎯 Summary

> To write a bot for this C2 server:
> - Connect TCP to `server_ip:9000`
> - Reply to `PING` with `PONG`
> - Send `HEARTBEAT` every 10 seconds
> - Handle additional commands as needed

Minimal, efficient, reliable.

---
Perfect — this is the **best moment** to write the full 📚 **Documentation**.  
I'll make it **proper**, as if it was a serious project README —  
**deep**, **organized**, **clear** — so you or anyone can understand and extend the project later.

Here we go:

---

# 📚 Modular Rust C2 Server — Documentation

---

## 📌 Overview

This project is a **modular, dynamic, authenticated Command-and-Control (C2) server** written in **Rust**.  
It allows users/admins to control bots over HTTP API, dispatching dynamic modules (commands) to bots.  
Bots connect via TCP and maintain open communication for receiving commands and sending heartbeats.

Architecture is fully **asynchronous**, **multi-client**, and **modular** — designed for **extensibility and real-world scaling**.

---

## 🗂️ Project Structure

```
src/
├── apis/          # HTTP API endpoints (send_command, list_bots, list_modules)
├── core/          # Core server state (connected bots, loaded modules, sessions)
├── communicating/ # TCP communication with bots (connect, heartbeat, messaging)
├── modules/       # Dynamically loaded modules (commands like "ping")
└── main.rs        # Entry point, loads everything, runs server
```

---

## 🚀 Server Startup Flow

1. **main.rs**:
   - Initializes `ServerState`.
   - Loads available `modules` (e.g., `ping`).
   - Starts:
     - TCP listener for bots (on `0.0.0.0:9000`).
     - HTTP API server for users/admins (on `0.0.0.0:8000`).

2. **Bots** connect automatically (no authentication) via TCP.

3. **Users/Admins** interact using the **API** (send commands, list bots, list modules).

---

## 📦 Core Components

### 1. ServerState (`core/state.rs`)
- Shared state across the server (wrapped in `Arc<Mutex<...>>`).
- Holds:
  - Connected bots (`bots` map).
  - Loaded modules (`modules` map).
  - User sessions (`sessions` map) — **(auth skipped for now)**.

```rust
pub struct ServerState {
    pub bots: Arc<Mutex<HashMap<Uuid, BotConnection>>>,
    pub modules: Arc<Mutex<HashMap<String, Arc<dyn C2Module + Send + Sync>>>>,
    pub sessions: Arc<Mutex<HashMap<String, SessionInfo>>>,
}
```

---

### 2. Modules System (`modules/`)
- Each module implements the `C2Module` trait:
  
```rust
#[async_trait]
pub trait C2Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, bot: Arc<BotConnection>, args: Vec<String>) -> anyhow::Result<()>;
}
```

- Loaded at server startup (`modules/loader.rs`).
- Automatically registered by name into `ServerState.modules`.

**Example module**: `ping` command sends `"PING"` to bots.

---

### 3. Bot Communication (`communicating/`)
- Bots connect over TCP (`0.0.0.0:9000`).
- Each bot is assigned a **UUID**.
- A lightweight **task** is spawned per bot to:
  - Listen for `PONG`, `HEARTBEAT`, and custom messages.
  - Detect disconnections.
- Bots send **HEARTBEAT** every 10 seconds to stay alive.

**BotConnection abstraction:**

```rust
pub struct BotConnection {
    pub connection: Arc<Mutex<TcpStream>>,
}

impl BotConnection {
    pub async fn send_message(&mut self, message: &str) -> Result<()> { ... }
}
```

---

### 4. API Server (`apis/`)
- Built with `axum` for fast async HTTP handling.
- Routes:

| Endpoint | Method | Description |
|:---------|:-------|:------------|
| `/send_command` | POST | Send a module command to a specific bot |
| `/list_bots` | POST | List all connected bots |
| `/modules` | POST | List all loaded modules |

Example `/send_command` request:

```json
{
  "session_token": "test",
  "bot_id": "uuid-of-bot",
  "command": "ping",
  "args": []
}
```

---

## ⚡ How Bots Work

Bots are very simple TCP clients:
- Connect to server.
- Wait for commands.
- React to commands (like `PING`) or send **HEARTBEAT** every 10s.

Example bot (simplified):

```rust
loop {
    select! {
        bytes = reader.read_line(&mut line) => { /* handle server command */ }
        _ = sleep(Duration::from_secs(10)) => { /* send HEARTBEAT */ }
    }
}
```

---

## 🔐 Authentication (Future Work)

- Currently **bypassed** for faster testing.
- Expected auth system:
  - Users login -> get `session_token`.
  - Admins have full access, users limited access.
  - `/send_command` and other sensitive APIs validate `session_token`.

---

## 🛠️ Example Usage

1. **Start server**:
   ```bash
   cargo run
   ```

2. **Start bot(s)**:
   ```bash
   cd simple_bot
   cargo run
   ```

3. **Control bots via API** (e.g., using `curl`):
   ```bash
   curl -X POST http://127.0.0.1:8000/list_bots -d '{}'
   curl -X POST http://127.0.0.1:8000/modules -d '{}'
   curl -X POST http://127.0.0.1:8000/send_command -H "Content-Type: application/json" -d '{
       "session_token": "test",
       "bot_id": "uuid-here",
       "command": "ping",
       "args": []
   }'
   ```

---

## 🌟 Future Improvements

| Feature | Priority |
|:--------|:---------|
| Real authentication system (login) | 🔥 |
| Broadcast commands to all bots | 🚀 |
| Store bot IP address and last heartbeat timestamp | 🛡️ |
| Hot-reload modules at runtime (dynamic libloading) | ⚡ |
| Web UI panel to manage bots visually | 🌐 |
| TLS encryption for API server | 🔒 |

---

# 🎯 Summary

This project is now a **full functional modular C2** with:
- Dynamic modules.
- Bot heartbeats.
- API control.
- Extensible for future exploits, payloads, management.

**Modern async Rust, built to scale and extend!**

