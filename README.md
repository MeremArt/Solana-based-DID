# Solana DID Program

This project implements a Decentralized Identifier (DID) program on the Solana blockchain using the Anchor framework. The functionality includes initializing DIDs, adding services to DIDs, and transferring control of DIDs.

**⚠️ Important Notice:**  
This code is not functional because the `sol-did-cpi` crate, which is integral to the implementation, is outdated and the associated repository is no longer operational. This project is preserved for educational and reference purposes.

---

## Features

1. **Initialize DID**
   - Sets up a new DID account with a specified authority.
2. **Add Service**
   - Adds a service to an existing DID account, including details such as service type and endpoint.
3. **Transfer Control**
   - Transfers the authority of a DID account to a new public key.

---

## File Structure

- **`src/instructions`**

  - `initialize_did.rs`: Handles DID initialization.
  - `add_service.rs`: Handles adding services to a DID.
  - `transfer_control.rs`: Handles transferring control of a DID to a new authority.

- **`src/state`**

  - `DidAccount`: Defines the account structure for a DID, including its authority.

- **`src/error`**

  - Defines custom error codes for validation and authorization.

- **`lib.rs`**
  - The main program entry point, connecting instructions and state definitions.

---

## Dependencies

- **Anchor Framework**: For Solana program development.
- **sol-did-cpi**: A CPI crate for interacting with Solana DIDs (no longer operational).

---

## Non-Functional Status

This project depends on the `sol-did-cpi` crate:

```toml
sol-did-cpi = { git = "https://github.com/identity-com/sol-did", branch = "main" }
```
