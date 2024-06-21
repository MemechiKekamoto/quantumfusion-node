# QuantumFusion Network Node

Welcome to the QuantumFusion Network Node repository. QuantumFusion Network leverages the power of HVM2 (Higher-Order Virtual Machine 2), a state-of-the-art parallel evaluator for Interaction Combinators. HVM2 offers near-ideal speedup in computational tasks, scaling from millions to billions of interactions per second.

## Vision

At QuantumFusion Network, we envision a world where computational limits are shattered. By leveraging HVM2 technology, we enable direct execution of raw mathematical constructs on GPUs, freeing AI algorithms from traditional constraints. This breakthrough allows for the development of revolutionary AI architectures and algorithms that were previously unimaginable.

While current AI algorithms are designed around GPU limitations, this technology opens the door to new possibilities, potentially leading to advancements far beyond today's large language models. Join us in pioneering the future of parallel computing, driving innovation, and transforming technology across industries.

## QuantumFusion(QF) Token

The QuantumFusion (QF) ERC-20 token is at the heart of our ecosystem. By holding QF tokens, you contribute to the development and expansion of the Blend, HVM, and QuantumFusion Network. Funds raised through QF token sales will be allocated to the following key areas:

- Blockchain Development: Accelerating the implementation of HVM2 technology into our blockchain infrastructure.
- Research and Innovation: Supporting the ongoing research and development efforts to enhance HVM2 and explore new applications.
- Community and Ecosystem Growth: Building a robust community of developers, researchers, and enthusiasts to drive adoption and innovation.

## Getting Started

Below you will find instructions on setting up, building, and running the QuantumFusion Node.

### Prerequisites

Ensure you have the following installed:

- [Rust](https://doc.rust-lang.org/)
- [Substrate](https://docs.substrate.io/)

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Run a Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/quantumfusion-node --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/quantumfusion-node purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/quantumfusion-node -ldebug --dev
```

## Contributing

We welcome contributions from the community. If you wish to contribute, please fork the repository and submit a pull request.

## License

This project is licensed under the terms of the MIT license.

## Join Us

Join us in pioneering the future of parallel computing, driving innovation, and transforming technology across industries. For more information, visit our [website](https://quantumfusion.pro/) and follow us on [GitHub](https://github.com/MemechiKekamoto/quantumfusion-node).