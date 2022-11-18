# fuel-nft-indexer
Indexer on fuel to index all nft contracts

## Prerequisites

- Clone the indexer 
```
git clone https://github.com/FuelLabs/fuel-indexer
```

- Install fuel toolchain from [here](https://fuellabs.github.io/fuel-indexer/master/getting-started/system-dependencies.html#system-requirements)
- Install app dependencies [here](https://fuellabs.github.io/fuel-indexer/master/getting-started/application-dependencies.html#application-dependencies)

## Execution

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
cargo run -bin fuel-indexer -- --manifest <path-to-manifest.yaml>
```
