# zokrates_credential
use case of on-chain credential verification using zokrates

## Overview
This project implements a credential system using ZoKrates, a toolbox for zkSNARKs on Ethereum. It demonstrates the process of creating verifiable credentials, generating proofs of those credentials, and verifying them on the blockchain. The system is divided into three main components: issuer, prover, and verifier, each playing a distinct role in the credential verification process.

## Structure
The project structure has evolved to include a CLI that facilitates interaction with the core logic of credential issuance, proof generation, and verification processes. Here is the updated structure:

```text
zokrates_credential
├── zokrates_credential_cli
│   ├── src
│   │   └── main.rs
│   └── Cargo.toml
├── zokrates_credential_core
│   ├── src
│   │   ├── credential.rs
│   │   ├── issuer.rs
│   │   ├── lib.rs
│   │   ├── prover.rs
│   │   └── verifier.rs
│   ├── zok
│   │   ├── create_hash.zok
│   │   ├── create_signature.py
│   │   └── verify_credential.zok
│   └── Cargo.toml
└── zokrates_crypto
    ├── src
    │   ├── babyjubjub.rs
    │   ├── eddsa.rs
    │   ├── field.rs
    │   ├── lib.rs
    │   └── utils.rs
    └── Cargo.toml
```
- **zokrates_credential_cli**: A command-line interface that enables users to easily interact with the core logic for issuing, proving, and verifying credentials.
- **zokrates_credential_core**: Contains the Rust modules that implement the core functionality of the credential system.
- **zokrates_crypto**: migrating the zokrates_pycrypto Library used for generating and verifying signatures. (ongoing)

## Setup Instructions
- Install ZoKrates according to the official documentation.
```bash
curl -LSfs get.zokrat.es | sh
```
- Clone this repository and navigate into the project directory.
```bash
git clone https://github.com/Ham3798/zokrates_credential.git
cd zokrates_credential
```
- Build the CLI using cargo build --release.
```bash
cargo build --release
```
- Optionally, add the path to the zokrates_credential_cli executable to your system's PATH environment variable for easier access.
```bash
export PATH=$PATH:/path/to/your/project/target/release
alias zokrates_credential='zokrates_credential_cli'
```

## CLI Usage
The `zokrates_credential_cli` provides a convenient way to access the core functionalities:

### Issuer
To perform issuer setup operations:
```sh
zokrates_credential issuer setup
```
To issue a new credential:
```sh
zokrates_credential issuer create_credential <credential_id> <name> <age> <student_number> <department> <signature_save_path>
```

### Prover
To perform prover setup operations:
```sh
zokrates_credential prover setup
```
To generate a proof for a given credential:
```sh
zokrates_credential prover create_proof <credential_path> <signature_path> <proving_key_path> <destination_path>
```

### Verifier
To run the ZoKrates setup for the verifier:
```sh
zokrates_credential verifier setup
```
To copy the proving key to a specified destination:
```sh
zokrates_credential verifier get_proving_key <destination_path>
```
To export a verifier smart contract:
```sh
zokrates_credential verifier get_verify_contract <destination_path>
```

## Key Management
The issuer's public and private keys are currently managed within the create_signature.py script. For debug purposes, the key is seeded with a specific value as shown below:

```text
# Seeded for debug purpose
key = FQ(1997011358982923168928344992199991480689546837621580239342656433234255379025)
```
This approach is used for demonstration and testing. In a real-world application, it's crucial to handle private keys securely, ensuring they are not hard-coded or exposed in your application code.


## Example
```bash
git clone https://github.com/Ham3798/zokrates_credential.git
cd zokrates_credential
cargo build --release
cd target/release
mkdir save
```
```bash
./zokrates_credential_cli issuer setup
```
```bash
./zokrates_credential_cli issuer create_credential 21 21 21 21 21 ./save
```
```bash
./zokrates_credential_cli verifier setup
```
```bash
./zokrates_credential_cli verifier get_proving_key ./save
```
```bash
./zokrates_credential_cli verifier get_verify_contract ./save
```
```bash
./zokrates_credential_cli prover setup
```
```bash
./zokrates_credential_cli prover create_proof ./save/credential.json ./save/signature ./save/proving.key ./save
```

## License
This project is licensed under the GPL-3.0 license - see the LICENSE file for details.
