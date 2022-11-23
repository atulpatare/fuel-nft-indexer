# fuel-nft-indexer

Indexer on fuel to index all nft contracts

## Prerequisites

- Install fuel toolchain
  from [here](https://fuellabs.github.io/fuel-indexer/master/getting-started/system-dependencies.html#system-requirements)
- Install app
  dependencies [here](https://fuellabs.github.io/fuel-indexer/master/getting-started/application-dependencies.html#application-dependencies)

## Execution

- Get the fuel indexer binary

```shell
# clone the repo
git clone https://github.com/FuelLabs/fuel-indexer

# build the indexer
cd fuel-indexer
cargo build --release

# copy the binary in the project dir bin folder
cp target/release/fuel-indexer <path_to_curre_project>/bin/
```

- Extra steps for Apple M1

```shell
brew install llvm
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
export CC=/opt/homebrew/opt/llvm/bin/clang
```

- Build the indexer

```shell
cargo build --release
```

- Snip the erant symbols (required for this version)

```shell
./scripts/wasm_snip.sh
```

- Execute the indexer

```shell
./bin/fuel-indexer --manifest manifest.yaml
```

- To run any of the scripts

```shell
# run from project root
./scripts/<script_name>.sh <arg0> <arg1> ...
```

- To query the data

```shell
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
