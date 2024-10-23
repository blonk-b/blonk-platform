<div align="center">
  <img src="./blonk.jpg" width="160" height="160">
  <h1> Blonk 🌟 - Radar Hackathon</h1>
  
  <h3>Blinks + Squads Multisig + Telegram</h3>
</div>

> If it blinks, it blonks

Blonk is a robust platform that streamlines multi-signature transaction management through an intuitive Telegram bot interface. By leveraging Dialect's blink protocol, Blonk enables seamless cross-platform transaction creation and management.

## What is Blonk? 🤔

Imagine Alice wants to initiate a treasury payment. Instead of navigating complex interfaces, she simply sends a blink to Blonk. The bot guides her through the process, collecting necessary details in a conversational manner. Once all information is gathered, Blonk creates a multi-signature transaction using Squads v3 and notifies all signers in a dedicated group.

Bob and Charlie, other signers in the group, can then review the transaction details and approve or reject with a single click. Once the required signature threshold is met, any group member can execute the transaction. [Learn more about the complete flow](USAGE.md).

## Architecture 🏗️

Blonk consists of two main components:

- **blonk_bot**: A Telegram bot built with Teloxide that handles user interactions
- **blonk_api**: A backend service powered by Axum that manages transaction logic and state

### Why Rust? 🦀

Our choice of Rust brings several advantages:

- **Security**: Memory safety guarantees and compile-time checks prevent common vulnerabilities
- **Performance**: Zero-cost abstractions and minimal runtime overhead
- **Reliability**: Strong type system and ownership model ensure robust operation
- **Future-proof**: Growing ecosystem and active community support

### Security First 🔒

Blonk utilizes Squads v3 for multi-signature functionality, a protocol that has undergone multiple successful security audits. This provides a battle-tested foundation for secure transaction management.

## Setting it up 🚀

The setup process involves configuring both the bot and API components, along with necessary environment variables and database setup. For detailed instructions, please refer to our [setup guide](SETUP.md).

## Development Status 📈

Currently implemented:

- Basic transaction creation flow
- Multi-signature approval/rejection
- Transaction execution
- Blink protocol support

### TODOs 📝

Stage 1: (UX)

- [ ] Improve Transaction Request Message UX (explorer links, signature tracking, status display, reload button)
- [ ] Handle loading state for all actions (create, approve, reject, execute)

Stage 2: (Mainnet)

- [ ] Handle errors and improve fault tolerance
- [ ] Priority fees and CU optimization

Stage 3: (Autonomy)

- [ ] Self-management of users and private keys
- [ ] Support creation of multisig with groups, changing threshold, adding/removing member

## Contributing 🤝

We welcome contributions! Feel free to:

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

Please ensure your code follows our style guidelines and includes appropriate tests.

---

Built with 💫 by the Blonk team
