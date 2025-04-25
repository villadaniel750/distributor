# 📦 Soroban Smart Contract: Distributor

This Soroban contract distributes tokens from a sender to multiple recipients using `transfer_from`. The sender must explicitly authorize the call with `require_auth()` and must have previously approved the contract to use their tokens.

---

## ✨ Function

### `distribute(env, xlm_sac, sender, recipients)`

Distributes tokens from `sender` to each `recipient` with the given amount.

- `xlm_sac`: Contract address of the token (e.g., tokenized XLM).
- `sender`: The address that must authorize the invocation and have approved token use.
- `recipients`: A vector of `(recipient_address, amount)` tuples.

---

## ✅ Validation Strategy: Backend vs Smart Contract

| Validation                                          | Location                  | Is Critical?  | Notes                                                    |
|-----------------------------------------------------|---------------------------|---------------|----------------------------------------------------------|
| `recipients` is not empty                           | ✅ Backend                | ❌ No         | Saves gas, avoids no-op calls.                           |
| Each `recipient` is a valid Stellar address         | ✅ Backend                | ❌ No         | Soroban `Address` type enforces valid input.             |
| Each `amount > 0`                                   | ✅ Backend + ✅ Contract  | ✅ **Yes**    | Prevents abuse, ensures safe logic.                      |
| Total `amount` ≤ sender’s balance                   | ✅ Backend                | ❌ No         | Improves UX, avoids failed transactions.                 |
| `sender` has approved this contract via `approve()` | ✅ Backend                | ❌ No         | Required for `transfer_from` to succeed.                 |
| `xlm_sac` points to a valid token contract          | ✅ Backend                | ❌ No         | Cannot be verified dynamically from contract.            |
| `require_auth(sender)`                              | ✅ Contract               | ✅ **Yes**    | Ensures the sender has signed the call (authentication). |

---

## ⚙️ Security Notes

- The contract is stateless.
- It enforces sender authentication via `require_auth()`.
- Uses `transfer_from(sender, sender, recipient, amount)` — this requires token `approve` to be called beforehand.
- Emits events for each transfer.

---

## 🚀 Build Instructions

1. Ensure you have Rust and Soroban CLI installed.
2. Compile the contract:

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
// From backend using Soroban JS SDK
contract.distribute({
  env,
  xlm_sac: tokenAddress,
  sender: userAddress,
  recipients: [
    [recipient1, amount1],
    [recipient2, amount2],
  ]
});
```

Make sure `userAddress`:
- Calls `approve(xlm_sac, contract_address, total_amount)` beforehand
- Signs the transaction to meet `require_auth()` check

---

## 🔐 License

MIT – use it freely and responsibly.
