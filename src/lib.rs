extern crate alloc;

use fuel_indexer_macros::indexer;
use fuels_core::*;

#[indexer(manifest = "../fuel-nft-indexer/manifest.yaml")]
pub mod nft_indexer_module {
    use super::*;

    pub fn handle_mint_event(event: MintEvent) {
        let MintEvent { owner, token_id_start, total_tokens } = event;
        match owner {
            Identity::Address(address) => {
                let mint = Mint {
                    id: token_id_start + total_tokens,
                    owner: address,
                    amount: total_tokens.try_into().unwrap(),
                    token_id_start: token_id_start.try_into().unwrap(),
                };
                mint.save();
            }
            _ => {}
        };
    }
}
