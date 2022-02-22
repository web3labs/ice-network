
## Installation

```
npm install
```
## Build

```
npm run build
```

## Run tests

In truffle-config.js file specify ethereum private keys for ICE network

```
const privKeys = (process.env.PRIVATE_KEYS) ? process.env.PRIVATE_KEYS.split(',') : 
[
  '<private_keys>',
];
```

```
npm run test
```

## Run integration tests

To run integration tests, first build ICE node

### ICE Node build
```
git clone https://github.com/web3labs/ice-substrate.git

cd ice-substrate

cargo build --release --locked --verbose --no-default-features --features manual-seal, rpc_binary_search_estimate
```
Then run tests
```
npm run ts-test
```
