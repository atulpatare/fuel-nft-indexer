extern crate alloc;
use fuel_indexer_macros::indexer;
use fuels_core::*;

#[indexer(manifest = "../fuel-nft-indexer/manifest.yaml")]
pub mod my_counter_index_module {

    //noinspection RsUnresolvedReference
    fn block_handler(_event: BlockData) {
        Logger::info("from event handler");
    }
}
