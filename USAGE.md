# Using Blonk

This guide explains how to use Blonk for managing multi-signature transactions through Telegram.

## Quick Commands

- `/help` - Shows available commands and basic usage information
- `/cancel` - Cancels the current transaction creation process

## How It Works

Let's walk through how Alice uses Blonk to create a treasury payment that requires approval from Bob and Charlie.

### Starting a Transaction

Alice starts by sending a blink URL to the Blonk bot. Important: Blonk handles one transaction at a time - you'll need to either complete or cancel the current transaction (using `/cancel`) before starting a new one.

The bot examines the blink and shows Alice what action it represents - maybe it's transferring 1000 USDC to a contributor's wallet. Alice can then review the details and decide whether to proceed with creating the transaction.

### Creating the Transaction

If the details look good, Alice clicks the interactive button to create the transaction. The bot handles all the complex parts - it creates the transaction under the configured multisig and sends a message to the group chat where Bob and Charlie are waiting.

### Group Review & Approval

The message appears in the group chat showing:

- What the transaction will do
- Interactive buttons for approve and reject
- Current approval count

Bob sees the transaction and clicks approve. Then Charlie reviews it and also approves. Note that once an approval is given, it can't be revoked - make sure to review carefully!

### Execution

Once enough people have approved (based on the multisig threshold), an execute button appears in the group message. Any member of the group can click it to execute the transaction.

After execution, the message updates to show that the transaction is complete.

## Example Flow

Here's a quick example of the conversation flow:

```
Alice → Bot: https://dialect.../blink/...
Bot: I found a blink requesting to transfer 1000 USDC to wallet 3d7x...
     Would you like to create this transaction?
     [Create Transaction] [Cancel]

Alice: [Clicks Create Transaction]
Bot → Group: New transaction needs approval
             Transfer 1000 USDC to 3d7x...
             [Approve] [Reject]

Bob: [Clicks Approve]
Group Message: Transfer 1000 USDC to 3d7x...
              Approvals: 1/2
              [Approve] [Reject]

Charlie: [Clicks Approve]
Group Message: Transfer 1000 USDC to 3d7x...
              Approvals: 2/2
              [Execute Transaction]

Bob: [Clicks Execute Transaction]
Group Message: ✅ Transaction executed
```

## Tips & Troubleshooting

- Always use `/help` if you're unsure about something
- If you need to start over, use `/cancel` to abort the current transaction
- Make sure to complete or cancel one transaction before starting another
- If the bot isn't responding to a blink URL, verify that:
  - Your bot instance is running
  - The API service is accessible
  - You're not in the middle of another transaction
- For approval issues, make sure:
  - You're a member of the whitelist
  - Your Telegram ID matches the whitelist entry
  - The multisig configuration matches your group
