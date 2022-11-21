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

    pub fn handle_transfer_event(event: TransferEvent) {
        let TransferEvent { from, to, token_id } = event;

        let from_address = match from {
            Identity::Address(address) => address,
            _ => panic!("done"),
        };
        let to_address = match to {
            Identity::Address(address) => address,
            _ => panic!("done"),
        };

        let transfer = Transfer {
            id: token_id,
            from_user: from_address,
            to_user: to_address,
        };
        transfer.save();
    }
}
