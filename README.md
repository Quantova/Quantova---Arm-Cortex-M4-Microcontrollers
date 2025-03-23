Compile for ARM Cortex-M4

I have added all the below files into pq_crypto_code folder for separate testing.

 crypto_kem/ → Key Exchange Mechanisms (Kyber, NTRU, etc.) optimized for Cortex-M4
 
 crypto_sign/ → Post-Quantum Digital Signatures (Dilithium, Falcon, etc.)
 
 mupq/ → Test framework that benchmarks post-quantum cryptography on Cortex-M4

 tests/ → Scripts that measure speed, RAM, and flash usage on Cortex-M4 hardware


Install the ARM toolchain:

bash
Copy
Edit
sudo apt install gcc-arm-none-eabi
Compile using:

bash
Copy
Edit
make PLATFORM=cortex-m4
2️⃣ Run Benchmarks on Your Cortex-M4 Board

Connect your ARM Cortex-M4 board (like STM32F4)

Run:

bash
Copy
Edit
python3 run_benchmarks.py
3️⃣ Analyze Performance

Compare key exchange and signature verification speeds on Cortex-M4

Optimize any slow-performing functions



Below are the Findings 

Key Areas Explored & Findings
1️. Benchmarking Framework Analysis
    • The pqm4 library includes a benchmarking toolset that measures speed, memory consumption, and code size for various post-quantum algorithms.
    • These benchmarks are crucial for evaluating the feasibility of post-quantum cryptography on embedded platforms.
    • The results help in comparing different PQC algorithms, allowing developers to select the best cryptographic primitives based on their hardware constraints.
    • Key metrics analyzed:
        ◦ Execution time (in clock cycles).
        ◦ RAM & Flash memory consumption.
        ◦ Key generation, encryption, and decryption performance.
 Findings:
    • PQC algorithms have higher computational costs than traditional ECC/RSA cryptography.
    • Lattice-based cryptographic schemes (e.g., Kyber, Dilithium) offer a good balance between security and efficiency.
    • Hash-based signature schemes (e.g., SPHINCS+) provide high security but at the cost of slower verification times and larger signature sizes.

2️. Integration of PQClean Implementations
    • pqm4 integrates cryptographic implementations from the PQClean project.
    • PQClean provides NIST PQC Standardization candidates with a focus on portability and security.
    • This integration ensures that the cryptographic implementations follow standardized practices and are vetted by the cryptographic community.
  Findings:
    • PQClean ensures that pqm4 adheres to best practices in post-quantum cryptography.
    • The modular design makes it easy to swap and compare different PQC algorithms.
    • Not all PQC algorithms are optimized for embedded platforms, and some require additional optimizations to improve performance.

3️. Post-Quantum Algorithms Tested on ARM Cortex-M4
I focused on two primary classes of post-quantum cryptographic algorithms implemented in pqm4:
	Key Encapsulation Mechanisms (KEMs): Used for secure key exchange.
    • Kyber (Lattice-based)
    • NTRU (Lattice-based)
    • FrodoKEM (Lattice-based)
  Digital Signature Schemes: Used for authentication and integrity verification.
    • Dilithium (Lattice-based)
    • SPHINCS+ (Hash-based)
    • Falcon (Lattice-based)
  Findings:
    • Kyber and Dilithium (both lattice-based) performed relatively well in terms of execution time and memory footprint.
    • SPHINCS+ had larger signature sizes and slower verification times, making it less suitable for constrained environments.
    • Falcon provided compact signatures but had higher complexity in key generation.
    • Hybrid cryptography (Classic + PQC) is feasible for transitioning to post-quantum security while maintaining compatibility with classical cryptographic systems.

4️.  Performance & Optimization Strategies for Cortex-M4
    • The pqm4 library optimizes finite field arithmetic and modular multiplication to enhance the efficiency of post-quantum algorithms.
    • It uses assembly-level optimizations and ARM Cortex-M4 DSP instructions for certain operations.
    • Key focus areas for optimization:
        ◦ Reducing stack and RAM usage for better memory efficiency.
        ◦ Optimizing polynomial arithmetic for lattice-based schemes (Kyber, Dilithium).
        ◦ Parallelizing operations where possible to speed up execution.
📌 Findings:
    • ARM Cortex-M4's limited RAM and Flash storage require careful algorithm selection.
    • Kyber and Dilithium provide a good tradeoff between speed and security, making them suitable candidates for PQC on embedded systems.
    • Further optimization at the assembly level could improve efficiency, particularly for lattice-based cryptographic schemes.
Conclusion
The mupq/pqm4 library is a powerful tool for implementing and benchmarking post-quantum cryptography on embedded devices. Through benchmarking, testing, and integration with PQClean, it provides insights into which PQC algorithms are most suitable for ARM Cortex-M4 microcontrollers. The next steps involve porting, optimizing, and contributing to the project while continuing research on hybrid cryptographic models and attack resistance strategies.
📌 GitHub Repository: https://github.com/mupq/pqm4
📌 Additional Reading:
    • NIST PQC Standardization
    • PQM4 Benchmarking Study

