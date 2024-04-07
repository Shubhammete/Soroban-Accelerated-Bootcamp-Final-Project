# Soroban-Accelerated-Bootcamp-Final-Project

Contract ID - CBBMIXVDQJB6TV36UKTQVBE2QT5DGR4OKJ7WZW6AQOEUVH5NAWXKZGAV

## Install Rust
<code> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh </code>

## Install Target
<code>rustup target add wasm32-unknown-unknown</code>

## Install the Soroban CLI
<code> cargo install --locked soroban-cli </code>

## Contract Build
<code>soroban contract build</code>


# Deployment

## Configuring the CLI for Testnet
<code>soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network" </code>

## Contract Deployment on testnet
  <code>soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet</code>



# About Project

Here we created Simple token on Soroban stellar testnet on which ew can interact it with contract ID
Whole smart contract is created using Rust and Soroban SDKs
We can Mint tokens and Burn as well. Also we can Freeze and Unfreeze the token for certain user



  
