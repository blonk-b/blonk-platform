# Setting up Blonk

This guide will walk you through setting up Blonk for development or deployment. We'll focus on the specific steps needed to get Blonk running, assuming you have basic development tools already installed.

## Prerequisites

- Rust and Cargo ([installation guide](https://www.rust-lang.org/tools/install))
- Git (for cloning the repository)
- Telegram account

## Setup Steps

1. Create a Telegram bot using BotFather and obtain its token
2. Setup a multisig with 2+ members using Squads v3
3. Create a Telegram group with your multisig members
4. Configure environment variables
5. Run the services

## Detailed Steps

### 1. Repository Setup

```bash
git clone https://github.com/your-org/blonk
cd blonk
```

### 2. Telegram Requirements

You'll need:

- A Telegram bot token (looks like `123456789:ABCdefGHIjklmNOPQRstuvwxyz`)
- A Telegram group with all multisig members
- The group's ID (a negative number)
- Telegram IDs for all group members

### 3. Configuration Setup

The easiest way to configure Blonk is through Cargo's environment configuration. Create the following structure:

```
blonk_bot/
└── .cargo/
    └── config.toml
```

Here's a template for your `config.toml`:

```toml
[env]
TELOXIDE_TOKEN = "your_bot_token_here"
WHITELIST = "[[\"telegram_id_1\",\"private_key_1\"],[\"telegram_id_2\",\"private_key_2\"]]"
API_BASE_URL = "http://127.0.0.1:3000"
MULTISIG_PUBKEY = "your_multisig_pubkey"
GROUP_CHAT_ID = "-your_group_chat_id"
```

### 4. Environment Variables Explained

| Variable            | Description                                                             |
| ------------------- | ----------------------------------------------------------------------- |
| `TELOXIDE_TOKEN`    | Your Telegram bot token                                                 |
| `WHITELIST`         | JSON array of [telegram_id, private_key] pairs for all multisig members |
| `API_BASE_URL`      | URL where blonk_api will run (default: `http://127.0.0.1:3000`)         |
| `MULTISIG_PUBKEY`   | Public key of your Squads v3 multisig. Not to be confused with the public key of the vault. You can find the public key of your Multisig here: Dashboard -> Info                                    |
| `GROUP_CHAT_ID`     | ID of your Telegram group                                               |

### 5. Running the Services

In separate terminal windows:

```bash
# Terminal 1
cd blonk_api
cargo run

# Terminal 2
cd blonk_bot
cargo run
```

## Troubleshooting

### Common Issues

1. **Bot not responding**: Check your `TELOXIDE_TOKEN` is correct
2. **API connection errors**: Verify `API_BASE_URL` matches where you're running blonk_api
3. **Whitelist format errors**: Double-check JSON formatting in WHITELIST env var

### Still stuck?

- Check the logs for both `blonk_api` and `blonk_bot`
- Ensure all private keys in whitelist correspond to multisig members
- Verify your multisig setup on Squads v3

## External Resources

- [Squads v3 Documentation](https://docs.squads.so/) - For multisig setup
- [Telegram Bot API](https://core.telegram.org/bots/api) - For bot-related issues
- [Rust Installation](https://www.rust-lang.org/tools/install) - If you need to install Rust

Need more help? Open an issue in our GitHub repository!
