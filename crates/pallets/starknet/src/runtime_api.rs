use sp_core::H256;

sp_api::decl_runtime_apis! {
    pub trait StarknetRuntimeApi {
        fn current_block_hash() -> H256;
        fn current_block() -> mp_starknet::block::Block;

		/// Returns the block with tx hashes of a given block.
    	fn get_block_with_tx_hashes(block_id: &BlockId) -> Result<MaybePendingBlockWithTxHashes, DispatchError>;
	}
}
