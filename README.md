<div align="center">
  <img src="./blonk.jpg" width="160" height="160">
  <h1>Blonk - Radar Hackathon</h1>
  
  <h3>Blinks + Squads Multisig + Telegram</h3>
</div>

Telegram bot that facilitates the management and approval of transactions through the integration of Blinks and Solana Multisig

## Getting Started

1. `git clone`
2. `cargo build`
3. Create a file `.cargo/config.toml` with the following env variable `PAYER_PRIVATE_KEY`
4. In `src/utils/consts.rs` add your Squads multisig public key
5. In your local environment add the following env variable `TELOXIDE_TOKEN` with your Telegram bot token. [Reference](https://github.com/teloxide/teloxide?tab=readme-ov-file#setting-up-your-environment)
6. `cargo run`