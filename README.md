# 📦 Soroban Smart Contract: Distributor

This Soroban contract distributes tokens from a sender to multiple recipients efficiently. It uses `token.transfer` directly, meaning the sender authorizes the transaction via `require_auth()`, without needing a prior `approve` step.

---

## ✨ Function

### `distribute(env, token_address, sender, recipients)`

Distributes tokens from `sender` to each `recipient` with the given amount.

- `token_address`: Address of the token contract (e.g., Native XLM SAC, USDC, etc.).
- `sender`: The address that must authorize the invocation.
- `recipients`: A vector of `(recipient_address, amount)` tuples.

---

## ✅ Validation Strategy: Backend vs Smart Contract

| Validation                                          | Location                  | Is Critical?  | Notes                                                    |
|-----------------------------------------------------|---------------------------|---------------|----------------------------------------------------------|
| `recipients` is not empty                           | ✅ Backend                | ❌ No         | Saves gas, avoids no-op calls.                           |
| Each `recipient` is a valid Stellar address         | ✅ Backend                | ❌ No         | Soroban `Address` type enforces valid input.             |
| Each `amount > 0`                                   | ✅ Backend + ✅ Contract  | ✅ **Yes**    | Prevents abuse (Contract panics if <= 0).                |
| Total `amount` ≤ sender’s balance                   | ✅ Backend                | ❌ No         | Improves UX, avoids failed transactions.                 |
| `token_address` points to a valid token contract    | ✅ Backend                | ❌ No         | Cannot be verified dynamically from contract.            |
| `require_auth(sender)`                              | ✅ Contract               | ✅ **Yes**    | Ensures the sender has signed the call (authentication). |

---

## ⚙️ Security & Efficiency Notes

- **Stateless**: The contract does not store data.
- **Authentication**: Enforces sender authentication via `require_auth()`.
- **Direct Transfer**: Uses `token.transfer(sender, recipient, amount)` instead of `transfer_from`. This is more gas-efficient and removes the need for allowances.
- **Batch Event**: Emits a single `distribute_batch` event at the end of the transaction with the total amount distributed, saving significant gas compared to per-transfer events.
- **Atomic**: If any transfer fails (e.g., insufficient balance), the entire transaction reverts.

---

## 🚀 Build Instructions

1. Ensure you have Rust and Soroban CLI installed.
2. Add the wasm target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Compile the contract:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

The output will be in:

```
target/wasm32-unknown-unknown/release/distributor.wasm
```

---

## 📜 Example Call

```js
// From backend using Soroban JS SDK (generating the XDR)
contract.distribute({
  env,
  token_address: tokenAddress, // Can be any Soroban-compatible token
  sender: userAddress,
  recipients: [
    [recipient1, amount1],
    [recipient2, amount2],
  ]
});
```

**Note**: The `userAddress` must sign the transaction. No prior `approve` call is needed.

---

## 🌐 Mainnet Deployment

**Contract ID**: `CC4CR7EV6IR3EPLHWQSFWPZP7WV7XZOQAXT25UX64JVH7DUUQA2UVSBJ`

---

## 🔐 License

MIT – use it freely and responsibly.
