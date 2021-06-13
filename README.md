# WhiteNoise For Substrate

A P2P privacy network [WhiteNoise](https://github.com/Evanesco-Labs/WhiteNoise) node template adapted to substrate.

## Getting Started

Follow the steps below to get started with the WhiteNoise network node.

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Build

The `cargo build` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

Specific parameters for WhiteNoise node:

- noise-bootstrap: Set the address of the WhiteNoise Network Bootstrap node. 

- noise-port: Set the local port to listen for WhiteNoise protocols.

Simply ignore these two parameters, if you want to run as an original substrate template node without WhiteNoise service.

## Run
The following shows how to start a WhiteNoise node as a service on a substrate node template. 
The WhiteNoise node will new an account from the same keypair as the substrate node template. 
### Multi-Node Local Network
These steps show how to start a local WhiteNoise Network and substrate multi-node consensus as the same time, 
refer to this [substrate tutoial](https://substrate.dev/docs/en/tutorials/start-a-private-network/)

First, start a node as a WhiteNoise network Bootstrap:
```sh
./node-template \
--base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
--noise-port 3331
```

Copy the node address in log, for later use. 
The Bootstrap node address is `/ip4/127.0.0.1/tcp/3331/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`, when log appears like the following:
```verilog
2021-06-08 18:13:55 local Multiaddress: /ip4/127.0.0.1/tcp/3331/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```


Then, start multiple nodes as WhiteNoise network relay nodes, make sure they use different ports.

Start a relay node:
```sh
./node-template \
--base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--noise-port 3332 \
--noise-bootstrap /ip4/127.0.0.1/tcp/3331/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

Start other nodes in the same way ...
```sh
./node-template \
--base-path /tmp/charlie \
  --chain local \
  --charlie \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--noise-port 3333 \
--noise-bootstrap /ip4/127.0.0.1/tcp/3331/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```


### Join WhiteNoise Network

Get official testnet bootstrap node [Address](./.testnet-boot.md). 
For example `/ip4/118.190.124.88/tcp/3331/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`.

Start a relay node to join this remote testnet:
```sh
./node-template --noise-bootstrap /ip4/118.190.124.88/tcp/3331/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp --noise-port 3332 
```
### RPC
The WhiteNoise node service share the original RPC service with substrate node template.
Set the substrate node template RPC port 9933. As an example the following command will gets WhiteNoise network node libp2p PeerIDs through RPC.
```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "get_main_nets", "id":1,"params":[10] }' 127.0.0.1:9933
```

## Substrate Node Template

This WhiteNoise node implementation is based on Substrate Node Template. Commands and parameters of the Substrate Node Template still remains. 
WhiteNoise service will not interfere with the original functions. 

Ignore these two parameters (noise-bootstrap, noise-port) to start the original Substrate Node Template.

Learn more about [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template).
