# ink! Quickstart Guide

This guide will walk you through setting up a simple ink! project and deploying it to a local development chain.
As an example, we will implement a simple PSP22 (similar to ERC20) token contract.
The main goal of this repository is to provide you a quickstart template that contains ready to use PSP22 implementation. 

## Before we start

Before we start, make sure you have the following prerequisites installed:
* Rust and Cargo: https://www.rust-lang.org/tools/install
* Download rust 1.74: `rustup toolchain  install 1.74`
* Install the cargo-contract utils. Detailed instruction on environemnt setup can be found here: https://use.ink/getting-started/setup and here https://github.com/paritytech/cargo-contract?tab=readme-ov-file#installation
* Install `substrate-contracts-node` https://use.ink/getting-started/setup#installing-substrate-contracts-node

To check what version of rust toolchain you are using, run `rustup show` and make sure that the default toolchain is set to 1.74.

Executing command `cargo contract` should print the following output:
```
Utilities to develop Wasm smart contracts

Usage: cargo contract <COMMAND>

Commands:
  new          Setup and create a new smart contract project
  build        Compiles the contract, generates metadata, bundles both together in a `<name>.contract` file
  check        Check that the code builds as Wasm; does not output any `<name>.contract` artifact to the `target/` directory
  upload       Upload contract code
  instantiate  Instantiate a contract
  call         Call a contract
  encode       Encodes a contracts input calls and their arguments
  decode       Decodes a contracts input or output data (supplied in hex-encoding)
  remove       Remove contract code
  info         Display information about a contract
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Build the contract

```bash
cargo contract build --release
```

This command will build the following for your contract: a Wasm binary, a metadata file (which contains the contract's ABI) and a .contract file which bundles both. 
This .contract file can be used to deploy your contract to a chain. If all goes well, you should see a target folder which contains these files:

```
target/
└── ink
    ├── ink_quickstart_psp22_prv.contract
    ├── ink_quickstart_psp22_prv.json
    └── ink_quickstart_psp22_prv.wasm
```

For more details about the output files, please refer to the official ink! docs: https://use.ink/getting-started/building-your-contract

## Deploy the contract

First start off with a local development chain. This can be done by running the following command: `substrate-contracts-node`

After that, you should get following output:
```
2024-01-10 02:43:19.004  INFO main sc_cli::runner: Substrate Contracts Node    
2024-01-10 02:43:19.004  INFO main sc_cli::runner: ✌  version 0.35.0-unknown    
2024-01-10 02:43:19.004  INFO main sc_cli::runner: ❤  by anonymous, 2021-2024    
2024-01-10 02:43:19.004  INFO main sc_cli::runner: 📋 Chain specification: Development    
2024-01-10 02:43:19.004  INFO main sc_cli::runner: 🏷  Node name: perfect-tin-6965    
2024-01-10 02:43:19.004  INFO main sc_cli::runner: 👤 Role: AUTHORITY    
2024-01-10 02:43:19.004  INFO main sc_cli::runner: 💾 Database: RocksDb at /tmp/substratewlqM30/chains/dev/db/full    
2024-01-10 02:43:19.525  INFO main sc_rpc_server: Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["*"] 
```

Remember to **not** close this terminal window, as it will stop the chain.

After that open https://contracts-ui.substrate.io/ in your browser and connect to the local development chain.
In your upper left corner make sure to select the "Local node" option.

![image](https://github.com/Smart-Beaver/.github/assets/8248700/b731d959-44ba-47ed-81ae-085f027d0110)

After that, click on the "Upload a new contract" button and select the .contract file from the target folder.

Detailed instructions with images can be found here: https://use.ink/getting-started/deploy-your-contract

## Interact with the contract

After you have deployed the contract, you can interact with it by clicking on the function name and filling in the required parameters.

More example: https://use.ink/getting-started/calling-your-contract