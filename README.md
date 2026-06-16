# Tip Jar

A Solana program (Anchor) where any user can spin up their own tip jar and collect SOL. Each jar is a PDA owned by its creator: anyone can tip into it, but only the owner can withdraw.

## How it works

Every tip jar is a Program Derived Address seeded by the owner's public key, so a wallet can have exactly one jar and its address is deterministic.

## Required instructions

| Instruction | Who can call | What it does |
|---|---|---|
| `initialize` | The owner | Creates the caller's `TipJar` PDA. Stores the owner and sets `total_tips = 0`. |
| `tip` | Anyone | Transfers `amount` lamports into a target jar via a System Program CPI; increments `total_tips`. |
| `withdraw` | The owner only | Withdraws lamports out of the jar back to the owner. |

## `TipJar` account

```rust
pub struct TipJar {
    pub owner: Pubkey,    // who owns this jar
    pub total_tips: u64,  // running total of lamports tipped
    pub bump: u8,         // canonical bump
}
```

## Things to demonstrate

- **PDA derivation** with seeds `[b"tip_jar", owner.key().as_ref()]` and store the **canonical bump**.
- Correct **space calculation** (discriminator + fields) in the `init` constraint.
- **Signer** check on `initialize` and `withdraw`.
- A **CPI to the System Program** for the SOL transfer in `tip`.
- Use **checked arithmetic** (`checked_add`) on `total_tips`.

## Deliverables

1. The program source in `programs/tip_jar/src/lib.rs`.
2. *(OPTIONAL)* A test file (`tests/`) proving:
   - `initialize` works,
   - a `tip` increases `total_tips`,
   - a non-owner **cannot** withdraw.

## Build & test

```bash
anchor build
anchor test
```