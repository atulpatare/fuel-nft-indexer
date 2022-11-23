# fuel-nft-indexer

Indexer on fuel to index all nft contracts

## Prerequisites

- Clone the indexer

```
git clone https://github.com/FuelLabs/fuel-indexer
```

- Install fuel toolchain
  from [here](https://fuellabs.github.io/fuel-indexer/master/getting-started/system-dependencies.html#system-requirements)
- Install app
  dependencies [here](https://fuellabs.github.io/fuel-indexer/master/getting-started/application-dependencies.html#application-dependencies)

## Execution

- Extra steps for Apple M1

```
brew install llvm
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
export CC=/opt/homebrew/opt/llvm/bin/clang
```

- Build the indexer

```
cargo build --release
```

- Snip the erant symbols (required for this version)

```
./wasm_snip.sh fuel-nft-indexer.wasm
```

- Execute the indexer

```
# build the indexer
cargo build --release

# run the binary
./target/release/fuel-indexer --manifest <path-to-manifest.yaml>
```

- To run any of the scripts

```
# run from project root
./scripts/<script_name>.sh <arg0> <arg1> ...
```

- To query the data
```
curl --location --request POST 'http://127.0.0.1:29987/api/graph/fuel_nft_indexer' \
--header 'Content-Type: application/json' \
--data-raw '{
    "query": "query { transfer { id from_user to_user } }",
    "params": ""
}'
```


## Testing

Testing with realtime data, use this NFT contract to interact and deploy.

[Repo](https://github.com/atulpatare/sway-nft)
