## Projects (Beginner to Advanced)
These are some project ideas generated from ChatGPT for beginner to advanced level to do while learning Rust

1. **CLI To-Do App (Beginner)**
    
- Use **structopt** or **clap** to handle command-line input.
- Store tasks in a local file using **serde** and **toml/json**.
- Implement basic CRUD operations (Add, Remove, List tasks).

2. **URL Shortener (Intermediate)**
- Build a simple HTTP server using axum or warp.
- Generate random short codes and map them to full URLs.
- Store mappings in a SQLite database with rusqlite.

3. **File Synchronizer (Intermediate)**
- Monitor a directory for file changes using notify.
- Automatically copy updated files to a backup directory.
- Handle multi-threading with tokio for async file operations.

4. **Simple Game with Bevy (Advanced)**
- Use bevy game engine to create a simple 2D game.
- Implement player movement and basic physics.
- Add simple AI behavior for enemies.

5. **Peer-to-Peer Chat Application (Advanced)**
- Use tokio and async-std to handle TCP communication.
- Support multiple clients in a decentralized manner.
- Implement message encryption with ring or rustls.