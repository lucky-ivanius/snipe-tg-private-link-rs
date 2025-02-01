# Telegram Private Link Sniper

A lightweight tool designed to monitor and capture Telegram invite link to private chat, implemented in Rust.

## Features

- Monitor Telegram channel messages for private chat links
- Auto join private chat

## Prerequisites

- Rust (latest stable version)
- Telegram API credentials

## Installation

1. Clone the repository:

```bash
git clone https://github.com/lucky-ivanius/snipe-tg-private-link-rs.git
cd snipe-tg-private-link-rs
```

2. Build the project:

```bash
cargo build --release
```

## Configuration

1. Obtain your Telegram API credentials (api_id and api_hash) from [my.telegram.org](https://my.telegram.org)
2. Create a `.env` file in the project root directory or simply do `cp .env.example .env`.
3. Configure your credentials in the env file (bot token or phone number, code, password, etc.)

```bash
API_ID=<your_api_id>
API_HASH=<your_api_hash>
CHANNEL_USERNAME=<target_channel_username>
```

## Usage

```bash
./target/release/snipe-tg-private-link-rs
```

> Disclaimer: when running for the first time, you will be prompted to authenticate your Telegram account. The authentication process (phone number, verification code, and 2FA password if enabled) is handled automatically by the Grammers client library. You only need to follow the prompts in the terminal.

#### Cleanup

To remove your Telegram session, simply delete the `auth.session` file.

## Disclaimer

This tool is for educational purposes only. Be sure to comply with Telegram's Terms of Service and API usage guidelines.