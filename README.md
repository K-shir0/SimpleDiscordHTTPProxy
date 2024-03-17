# Simple Discord HTTP Proxy Bot

This is a simple Discord bot that sends HTTP requests to the server and can send messages to any Discord channel.

## Features

* Send HTTP requests to the server and send the message to the Discord channel
  (Simply send the channel name and it will be fuzzily resolved.)

```bash
# Example (Channel)
curl -X POST --location "http://localhost:8080" \
    -H "accept: application/json" \
    -H "content-type: application/json" \
    -d '{
            "name": "general",
            "message": "hello world"
        }'
        
# Example (Thread)
curl -X POST --location "http://localhost:8080" \
    -H "accept: application/json" \
    -H "content-type: application/json" \
    -d '{
            "type": "thread",
            "name": "thread-name",,
            "message": "hello world",
        }'
   
# Example (Announcement)  
curl -X POST --location "http://localhost:8080" \
    -H "accept: application/json" \
    -H "content-type: application/json" \
    -d '{
            "type": "news",
            "name": "news-name",,
            "message": "hello world",
        }'
```

## Build

* needs Rust

```bash
cargo build --release
```

## Run

```bash
export DISCORD_TOKEN=...
export DISCORD_SERVER_ID=...
export HTTP_SERVER_HOST=... (Optional, default: 0.0.0.0)
export HTTP_SERVER_PORT=... (Optional, default: 8080)
./target/release/simple-discord-http-proxy-bot
```