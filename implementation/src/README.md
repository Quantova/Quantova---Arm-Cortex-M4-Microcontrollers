Build & Run
Compile the Rust code for Cortex-M4:

cargo build --target=thumbv7em-none-eabihf
Run using QEMU Cortex-M4 simulator:

qemu-system-arm -cpu cortex-m4 -nographic -kernel target/thumbv7em-none-eabihf/debug/rust-pqm4-ffi


1. Setting Up pqm4 in Rust Using FFI
Ensure pqm4 C library is compiled and available for Rust to link against.

Modify the build.rs file to properly link pqm4 with Rust using cc or bindgen.

Write a Rust wrapper to expose pqm4’s key encapsulation (Kyber) and digital signature (Dilithium).

2. Benchmarking Performance on Cortex-M4
Deploy the code on an ARM Cortex-M4 board (e.g., STM32F4, NXP i.MX RT).

Measure execution time, RAM usage, and power consumption for pqm4’s Kyber and Dilithium.

Compare pqm4-based PQC vs. ECC (Elliptic Curve Cryptography).

3. Implementing PQC for Blockchain Transactions
Substrate-Based Blockchain

Modify the crypto primitives to support pqm4’s post-quantum keys.

Implement quantum-resistant key signing (Dilithium) for transactions.

EVM-Based Blockchain (Ethereum & L2s)

Develop a smart contract-compatible PQC verification scheme.

Create a Rust SDK for Ethereum clients to interact with pqm4-verified transactions.

4. Alternative Rust-Native PQC Investigation
Explore pqcrypto-rust or rust-kem to avoid C dependencies.

Compare pqm4 + FFI vs. native Rust PQC in terms of security, performance, and ease of use.