# zokrates_credential
use case of on-chain credential verification using zokrates

## Overview
This project implements a credential system using ZoKrates, a toolbox for zkSNARKs on Ethereum. It demonstrates the process of creating verifiable credentials, generating proofs of those credentials, and verifying them on the blockchain. The system is divided into three main components: issuer, prover, and verifier, each playing a distinct role in the credential verification process.

## Structure
The project structure is as follows:

```text
zokrates_credential
├── RustZoCrypto
│   └── src
│       ├── babyjubjub.rs
│       ├── eddsa.rs
│       ├── field.rs
│       ├── lib.rs
│       └── utils.rs
├── src
│   ├── credential.rs
│   ├── issuer.rs
│   ├── lib.rs
│   ├── prover.rs
│   └── verifier.rs
└── zok
    ├── issuer
    │   ├── create_hash.zok
    │   └── create_signature.py
    ├── prover
    │   └── verify_credential.zok
    └── verifier
        └── verify_credential.zok
```
- **RustZoCrypto**: migrating the zokrates_pycrypto Library used for generating and verifying signatures. (ongoing)
- **src**: Contains Rust modules for the credential system's core functionality.
- **zok**: ZoKrates programs for credential hashing, proof generation, and verification.

## Usage
### Issuer
- **create_credential.rs**: Generates a cryptographic hash of the credential details and serializes them into a JSON format.
- **create_signature.py**: Creates a digital signature for the serialized credential using the RustZoCrypto library. (To be replaced by RustZoCrypto in the future)

### Prover
- **prover.rs**: Loads the credential and signature, generates a witness, and then produces a zkSNARK proof asserting the validity of the credential without revealing its content.

### Verifier
- **verifier.rs**: Compiles the ZoKrates verification program and generates a smart contract that can be used to verify the proof on the Ethereum blockchain.

## Setup Instructions
- Install ZoKrates following the official documentation.
- Install zokrates_pycrypto. (To be removed later)
```bash
curl -LSfs get.zokrat.es | sh
pip3 install zokrates_pycrypto 
```

- Execute the tests in issuer.rs, prover.rs, and verifier.rs to perform the credential issuance, proof generation, and verification process.

## License
This project is licensed under the GPL-3.0 license - see the LICENSE file for details.