//! Starknet RPC API trait and types
//!
//! Starkware maintains (a description of Starknet API)[https://github.com/starkware-libs/starknet-specs/blob/master/api/starknet_api_openrpc.json] using the openRPC specification.
//! This crate uses `jsonrpsee` to define such an API in Rust terms.

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};

pub type FieldElement = String;
pub type BlockNumber = u64;
pub type BlockHash = FieldElement;

/// A tag specifying a dynamic reference to a blocl
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct BlockHashAndNumber {
    pub block_hash: FieldElement,
    pub block_number: BlockNumber,
}

/// A block hash, number or tag
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum BlockId {
    BlockHash(FieldElement),
    BlockNumber(BlockNumber),
    BlockTag(BlockTag),
}

pub enum MaybePendingBlockWithTxHashes {
	Block(BlockWithTxHashes),
	PendingBlock(PendingBlockWithTxHashes),
}

/// The block object.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockWithTxHashes {
	pub status: BlockStatus,
	#[serde_as(as = "UfeHex")]
	pub block_hash: FieldElement,
	/// The hash of this block's parent
	#[serde_as(as = "UfeHex")]
	pub parent_hash: FieldElement,
	/// The block number (its height)
	pub block_number: u64,
	/// The new global state root
	#[serde_as(as = "UfeHex")]
	pub new_root: FieldElement,
	/// The time in which the block was created, encoded in Unix time
	pub timestamp: u64,
	/// The Starknet identity of the sequencer submitting this block
	#[serde_as(as = "UfeHex")]
	pub sequencer_address: FieldElement,
	/// The hashes of the transactions included in this block
	#[serde_as(as = "Vec<UfeHex>")]
	pub transactions: Vec<FieldElement>,
}

/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingBlockWithTxHashes {
	/// The hashes of the transactions included in this block
	#[serde_as(as = "Vec<UfeHex>")]
	pub transactions: Vec<FieldElement>,
	/// The time in which the block was created, encoded in Unix time
	pub timestamp: u64,
	/// The Starknet identity of the sequencer submitting this block
	#[serde_as(as = "UfeHex")]
	pub sequencer_address: FieldElement,
	/// The hash of this block's parent
	#[serde_as(as = "UfeHex")]
	pub parent_hash: FieldElement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStatus {
	#[serde(rename = "PENDING")]
	Pending,
	#[serde(rename = "ACCEPTED_ON_L2")]
	AcceptedOnL2,
	#[serde(rename = "ACCEPTED_ON_L1")]
	AcceptedOnL1,
	#[serde(rename = "REJECTED")]
	Rejected,
}

/// Starknet rpc interface.
#[rpc(server, namespace = "starknet")]
pub trait StarknetRpcApi {
    /// Get the most recent accepted block number
    #[method(name = "blockNumber")]
    fn block_number(&self) -> RpcResult<BlockNumber>;

    /// Get the most recent accepted block hash and number
    #[method(name = "blockHashAndNumber")]
    fn block_hash_and_number(&self) -> RpcResult<BlockHashAndNumber>;

    /// Get the number of transactions in a block given a block id
    #[method(name = "getBlockTransactionCount")]
    fn get_block_transaction_count(&self, block_id: BlockId) -> RpcResult<u128>;

	/// Get block information with transaction hashes given the block id
	#[method(name = "getBlockWithTxHashes")]
	fn get_block_with_tx_hashes(&self, block_id: BlockId) -> RpcResult<MaybePendingBlockWithTxHashes>;
}
