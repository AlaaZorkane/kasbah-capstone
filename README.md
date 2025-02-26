# Kasbah: Privacy-Preserving Token Protocol for Solana

Kasbah is a privacy-focused confidential token protocol built on Solana that enables secure, private token transfers using zero-knowledge proofs.

## Overview

Kasbah allows users to:
- Deposit tokens into commitment pools with zero-knowledge proofs
- Generate off-chain receipts that can be sent to recipients
- Redeem tokens without revealing the connection between deposit and withdrawal transactions

This ensures transaction privacy while maintaining the security and auditability required for blockchain applications.

## Architecture

### Components

1. **Solana Program (`programs/kasbah`)**
   - Core protocol implementation with treasury and commitment pools
   - Handles deposit and redemption operations

2. **Zero-Knowledge Circuits (`circuits/`)**
   - Implements `KasbahCommitmentCircuit` for ZK proofs
   - Uses Poseidon hash functions for commitments and Merkle tree verification

3. **WebAssembly Module (`wasm/`)**
   - Client-side ZK proof generation
   - Handles format conversions for proofs

4. **Verifier (`verifier/`)**
   - On-chain verification of Groth16 proofs
   - Adapted from Lightprotocol's implementation

### How it Works

1. **Deposit Flow**:
   - User deposits tokens, creating a commitment
   - Commitment is added to an on-chain commitment pool
   - User generates a receipt with recipient information

2. **Redemption Flow**:
   - Recipient uses the receipt to generate a zero-knowledge proof
   - Proof is submitted to the Kasbah program
   - System verifies the proof and sends tokens to recipient
   - Nullifiers prevent double-spending

## Technology Stack

- **Blockchain**: Solana
- **Programming Languages**: Rust, TypeScript
- **Smart Contract Framework**: Anchor
- **Zero-Knowledge Proofs**: Groth16 proofs
- **ZK Circuit Language**: Circom
- **Client-Side Proof Generation**: WebAssembly (WASM)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0+)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (1.16.0+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (0.28.0+)
- [Node.js](https://nodejs.org/) (16+)
- [Circom](https://docs.circom.io/getting-started/installation/) (2.0.0+)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/kasbah-capstone.git
   cd kasbah-capstone
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the project:
   ```bash
   anchor build
   ```

4. Build the WASM module:
   ```bash
   cd wasm
   cargo build --release
   cd ..
   ```

### Testing

Run the test suite to verify the installation:

```bash
anchor test
```

## Development

### Building Circuits

To rebuild the circuits:

```bash
cd circuits
circom ksb.circom --r1cs --wasm
```

### Generating TypeScript Clients

```bash
npm run generate-clients
```

## Security

This project implements several security measures:

- **Zero-Knowledge Proofs**: For privacy-preserving verification
- **Nullifiers**: To prevent double-spending
- **Commitment Pools**: To organize and secure deposit data
- **On-chain Verification**: Using Solana's bn254 verification syscall

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Adapted verifier from [Lightprotocol](https://github.com/lightprotocol)'s implementation
- Built on [Solana](https://solana.com/) blockchain
- Uses [Anchor](https://www.anchor-lang.com/) framework for Solana development