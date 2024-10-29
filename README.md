# <h1 align="center"> Tangle's Sub0 Blueprint Challenges 🌐 </h1>

## Overview

This template is our Hello World Tangle Blueprint. The documentation below should guide you through the process of creating a new blueprint and deploying it to the Tangle network. This repo is specifically geared towards giving you ideas and challenges to get started building restaking services for Tangle and the broader Substrate ecosystem.

Tangle is a Substrate based Layer-1 blockchain that is purpose built for restaking and offchain services. Tangle will be able to leverage any Polkadot and Substrate ecosystem asset to restake on services and earn rewards from these services. This blueprint is a simple example of how you can create a restaking service on Tangle.

Blueprints are simply templates for infrastructure services that can do anything from running a simple web server to a complex decentralized application. They are designed to be easy to use and deploy, and can be customized to suit your needs. By deploying Blueprints to Tangle, your customers can leverage Tangle's operator set and restaking assets to secure the instances of your Blueprints.

For more details, please refer to the [project documentation](https://docs.tangle.tools/developers/blueprints).

### Event Listener Challenges
Tangle's Gadget SDK is an event-driven framework that allows developers to build custom event listeners that can listen to pretty much anything! When your custom event listeners trigger, your **`#[job(...)]`** will execute. This is a powerful feature that allows you to build custom services that can listen to any event on the Tangle network, any EVM, parachain, or external API.

1. **`[Event Challenge 1]`** Create a custom event listener that listens to an arbitrary Substrate/Parachain RPC and triggers a job when a new block is minted. The job should log the block number and the block hash to the console.
2. **`[Event Challenge 2]`** Create a custom event listener that checks a weather API (e.g. OpenWeatherMap) every hour for a specified location. The job should trigger and log a message to the console when rain is detected in the forecast. The listener should use proper error handling for API failures and implement a configurable location parameter.
3. **`[Event Challenge 3]`** Create a custom event listener that monitors a sequence of connected EVM events and triggers a job only when the full sequence is detected. For example:
   - Create a smart contract that emits a sequence of events (e.g. `Event1`, `Event2`, `Event3`) over a sequence of transactions (e.g. `Tx1`, `Tx2`, `Tx3`)
   - The events should be linked to one another, i.e. `Event2` contains an identifier for data in `Event1`, and `Event3` contains an identifier for data in `Event2`, and so on.
   - Implement an event listener that listens for the full sequence of events and triggers a job when the sequence is detected.
   - The job should log the detected sequence to the console and include proper error handling for event sequence detection

### Blueprint Challenges
Many of the these blueprint challenges might leverage useful tools from our SDK like
- [Docker containers and executions](https://github.com/tangle-network/gadget/blob/main/sdk/src/docker.rs)
  - An example of a Blueprint using this is our [Obol DVT Blueprint](https://github.com/tangle-network/obol-dvt-blueprint)

1. **`[Blueprint Challenge 1] (200 TNT)`** Build a Tangle blueprint that hosts an IPFS node and pins content. The blueprint should:
   - Initialize and run an IPFS node using the IPFS HTTP API
   - Implement a job that accepts content (files/data) and pins it to IPFS
   - Return and log the IPFS hash (CID) of the pinned content
   - Include proper error handling for node initialization and pinning operations
   - Allow configuration of IPFS node parameters (e.g., storage location, gateway ports)
   - Implement basic node health monitoring and logging
2. **`[Blueprint Challenge 2] (100 TNT)`** Build a simple price oracle data feed blueprint. The blueprint should:
   - Accept user requests for price data for specific cryptocurrency pairs
   - Fetch current price data from a price API (e.g. CoinGecko)
   - Post the fetched price data to the Tangle blockchain
   - Include basic error handling for API failures
3. **`[Blueprint Challenge 3] (200 TNT)`** Build a blueprint that runs an SSV Network operator. The blueprint should:
   - Set up and configure an SSV Network operator node
   - Register the operator on the SSV Network
   - Monitor validator performance and network health
   - Implement proper error handling and logging
   - Include configuration options for network parameters and performance tuning
4. **`[Blueprint Challenge 4] (20 TNT)`** Build a blueprint that runs a simple web server. The blueprint should:
   - Initialize and run a simple web server using a Rust web framework (e.g. Actix)
   - Serve a basic HTML page with a message and a link to the Tangle Explorer
   - Include basic error handling for server initialization and request handling
   - Implement logging and monitoring for server health
5. **`[Blueprint Challenge 5] (200 TNT)`** Build a blueprint that extends our [Gaia AI Agent Blueprint](https://github.com/tangle-network/gaia-ai-agent-template) to do something new. The blueprint could:
   - Implement a new job that performs a custom AI task (e.g. image recognition, text analysis)
   - Extends our API server with API keys and authentication
6. **`[Blueprint Challenge 6] (500 TNT)`** Build a blueprint that deploys programs into a TEE secure enclave. The blueprint should:
   - Set up and configure a TEE environment (e.g. Intel SGX, AMD SEV)
   - Implement secure program deployment and attestation
   - Verify program integrity and authenticity before deployment
   - Enable secure data input/output between the enclave and external environment
   - Include proper error handling
7. **`[Blueprint Challenge 6] (1000 TNT)`** Build a blueprint that extends our Obol DVT blueprint to enable liquid staking. The blueprint should:
   - Extend the Obol DVT blueprint to run a validator cluster
   - Deploy and host a dApp frontend that allows users to:
     - Connect their wallet and deposit ETH for liquid staking
     - View their staking positions and rewards
     - Withdraw their staked ETH and rewards
   - Implement smart contracts for:
     - Liquid staking token (LST) minting/burning
     - Reward distribution
     - Withdrawal queue management
   - Include proper error handling and monitoring for:
     - Smart contract interactions
     - User deposits/withdrawals
     - DVT cluster performance
   - Implement logging and alerts for key events
   - Allow configuration of:
     - Fee parameters
     - Frontend customization
   
## 📚 Prerequisites

Before you can run this project, you will need to have the following software installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [Forge](https://getfoundry.sh)
- [Tangle](https://github.com/tangle-network/tangle?tab=readme-ov-file#-getting-started-)

You will also need to install [cargo-tangle](https://crates.io/crates/cargo-tangle), our CLI tool for creating and
deploying Tangle Blueprints:

To install the Tangle CLI, run the following command:

> Supported on Linux, MacOS, and Windows (WSL2)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tangle-network/gadget/releases/download/cargo-tangle-v0.1.2/cargo-tangle-installer.sh | sh
```

Or, if you prefer to install the CLI from crates.io:

```bash
cargo install cargo-tangle --force # to get the latest version.
```

## 🚀 Getting Started

Once `cargo-tangle` is installed, you can create a new project with the following command:

```sh
cargo tangle blueprint create --name <project-name>
```

and follow the instructions to create a new project.

## 🛠️ Development

Once you have created a new project, you can run the following command to start the project:

```sh
cargo build
```

to build the project, and

```sh
cargo tangle blueprint deploy
```

to deploy the blueprint to the Tangle network.

## 📜 License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 📬 Feedback and Contributions

We welcome feedback and contributions to improve this blueprint.
Please open an issue or submit a pull request on
our [GitHub repository](https://github.com/tangle-network/blueprint-template/issues).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
