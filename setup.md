# Frontier Node Template Setup


### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/frontier-template-node -h
```

### Generate your account keys
1. In the project root directory generate a random secret phrase and keys by running the following command:
```bash
./target/release/frontier-template-node key generate --scheme Sr25519 --password-interactive
```
2. Type a password for the generated keys.
The command generates keys and displays output similar to the following:
```bash
Secret phrase:  pig giraffe ceiling enter weird liar orange decline behind total despair fly
Secret seed:       0x0087016ebbdcf03d1b7b2ad9a958e14a43f2351cd42f2f0a973771b90fb0112f
Public key (hex):  0x1a4cc824f6585859851f818e71ac63cf6fdc81018189809814677b2a4699cf45
Account ID:        0x1a4cc824f6585859851f818e71ac63cf6fdc81018189809814677b2a4699cf45
Public key (SS58): 5CfBuoHDvZ4fd8jkLQicNL8tgjnK8pVG9AiuJrsNrRAx6CNW
SS58 Address:      5CfBuoHDvZ4fd8jkLQicNL8tgjnK8pVG9AiuJrsNrRAx6CNW
```
You now have the Sr25519 key for producing blocks using `aura` for one node. In this example, the Sr25519 public key for the account is:
`5CfBuoHDvZ4fd8jkLQicNL8tgjnK8pVG9AiuJrsNrRAx6CNW`

```bash
./target/release/frontier-template-node key inspect --password-interactive --scheme Ed25519 0x0087016ebbdcf03d1b7b2ad9a958e14a43f2351cd42f2f0a973771b90fb0112f
```
Type the password you used to the generated keys.
The command displays output similar to the following:

```bash
Secret Key URI `0x0087016ebbdcf03d1b7b2ad9a958e14a43f2351cd42f2f0a973771b90fb0112f` is account:
Secret seed:       0x0087016ebbdcf03d1b7b2ad9a958e14a43f2351cd42f2f0a973771b90fb0112f
Public key (hex):  0x2577ba03f47cdbea161851d737e41200e471cd7a31a5c88242a527837efc1e7b
Account ID:        0x2577ba03f47cdbea161851d737e41200e471cd7a31a5c88242a527837efc1e7b
Public key (SS58): 5CuqCGfwqhjGzSqz5mnq36tMe651mU9Ji8xQ4JRuUTvPcjVN
SS58 Address:      5CuqCGfwqhjGzSqz5mnq36tMe651mU9Ji8xQ4JRuUTvPcjVN
```
You now have the Ed25519 key for finalizing blocks using `grandpa` for one node. In this example, the Ed25519 public key for the account is:
`5CuqCGfwqhjGzSqz5mnq36tMe651mU9Ji8xQ4JRuUTvPcjVN`

### Generate set of keys for other validator nodes
Other participants also need to generate their keys to join your private network and provide you with the SS58 Address for Sr25519 & Ed25519.
### Create a custom chain specification
After you generate the keys to use with your blockchain, you are ready to create a custom chain specification using those key pairs then share your custom `chain specification` with trusted network participants called `validators`.
To enable others to participate in your blockchain network, you should ensure that they generate their own keys. If other participants have generated their key pairs, you can create a custom chain specification to replace the local chain specification that you used previously.

#### Modify an existing chain specification
1. Export the local chain specification to a file named customSpec.json by running the following command:
```bash
./target/release/frontier-template-node build-spec --disable-default-bootnode --chain local > customSpec.json
```
2. Open the customSpec.json file in a text editor.
3. Modify `aura` field to specify the nodes with the authority to create blocks by adding the Sr25519 SS58 address keys for each network participant.
```bash
"aura": {
    "authorities": [
      "5CfBuoHDvZ4fd8jkLQicNL8tgjnK8pVG9AiuJrsNrRAx6CNW",
      "5EJPj83tJuJtTVE2v7B9ehfM7jNT44CBFaPWicvBwYyUKBS6"
    ]
  },
```
4. Modify the `grandpa` field to specify the nodes with the authority to finalize blocks by adding the Ed25519 SS58 address keys for each network participant.
```bash
"grandpa": {
    "authorities": [
      [
        "5CuqCGfwqhjGzSqz5mnq36tMe651mU9Ji8xQ4JRuUTvPcjVN",
        1
      ],
      [
        "5FeJQsfmbbJLTH1pvehBxrZrT5kHvJFj84ZaY5LK7NU87gZS",
        1
      ]
    ]
  },
```
Note that there are two data values for the authorities field in the grandpa section. The first value is the address key. The second value is used to support `weighted votes`. In this example, each validator has a weight of 1 vote.
#### Convert the chain specification to use the raw format
1. Convert the customSpec.json chain specification to the raw format with the file name customSpecRaw.json by running the following command:
```bash
./target/release/frontier-template-node build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```
#### Share the chain specification with others
If you are creating a private blockchain network to share with other participants, ensure that only one person creates the chain specification and shares the resulting raw version of that specification—for example, the customSpecRaw.json file—with all of the other validators in the network.

Because the Rust compiler produces optimized WebAssembly binaries that aren't deterministically reproducible, each person who generates the Wasm runtime produces a slightly different Wasm blob. To ensure determinism, all participants in the blockchain network must use exactly the same raw chain specification file

### Launch the Private Network
### Start the First Node
1. Purge old chain data
```bash
./target/release/frontier-template-node purge-chain --base-path /tmp/node01 --chain local -y
```
2. Start the first node using the custom chain specification by running the following command:
```bash
./target/release/frontier-template-node \
--base-path /tmp/node01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode01
```
#### Add keys to the keystore
After you start the first node, no blocks are yet produced. The next step is to add two types of keys to the keystore for each node in the network. Each participant should do it on his machine.

For each node:

 - Add the `aura` authority keys to enable block production.

 - Add the `grandpa` authority keys to enable block finalization.

1. Insert the aura secret key generated from the key subcommand by running a command similar to the following:
```bash
./target/release/frontier-template-node key insert --base-path /tmp/node01 \
--chain customSpecRaw.json \
--suri <your-secret-key> \
--password-interactive \
--key-type aura \
--scheme sr25519
```
2. Insert the grandpa secret key generated from the key subcommand by running a command similar to the following:
```bash
./target/release/frontier-template-node key insert --base-path /tmp/node01 \
--chain customSpecRaw.json \
--suri <your-secret-key> \
--password-interactive \
--key-type gran \
--scheme ed25519
```
3. Verify that your keys are in the keystore for node01 by running the following command:
```bash
ls /tmp/node01/chains/local_testnet/keystore
```
The command displays output similar to the following:
```bash
617572611441ddcb22724420b87ee295c6d47c5adff0ce598c87d3c749b776ba9a647f04
6772616e1441ddcb22724420b87ee295c6d47c5adff0ce598c87d3c749b776ba9a647f04
```
### Enable other participants to join as validators
1. Purge old chain data, if needed, by running the following command:
```bash
./target/release/frontier-template-node purge-chain --base-path /tmp/node02 --chain local -y
```
2. Start a second blockchain node by running the following command:
```bash
./target/release/frontier-template-node \
--base-path /tmp/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/<ip_address>/tcp/<port>/p2p/<node_id>
```
Note: for the <ip_address> put the ip_address of the server where the bootnode is running, and for <node_id> put the node identity which would be shown in the command line when you start the bootnode.
#### Add the validators keys to the keystore
Add the validators keys to the keystore as in the above example for adding keys for bootnode (first node).

3. Restart the validator node with the above command.

4. Do the same for other validators.
