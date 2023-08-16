
//! Autogenerated weights for `pallet_order_book`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Theas-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// /Users/thealeake/centrifuge-repos/centrifuge-chain/target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_order_book
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/pallet_order_book.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_order_book`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_order_book::WeightInfo for WeightInfo<T> {
	/// Storage: OrderBook OrderIdNonceStore (r:1 w:1)
	/// Proof: OrderBook OrderIdNonceStore (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:1 w:1)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrderBook AssetPairOrders (r:1 w:1)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8000070), added: 8002545, mode: MaxEncodedLen)
	/// Storage: OrderBook Orders (r:0 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn create_order_v1() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1585`
		//  Estimated: `8017313`
		// Minimum execution time: 56_000 nanoseconds.
		Weight::from_parts(57_000_000, 8017313)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: OrderBook Orders (r:1 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:1 w:1)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook AssetPairOrders (r:1 w:1)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8000070), added: 8002545, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn user_cancel_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1464`
		//  Estimated: `8012936`
		// Minimum execution time: 48_000 nanoseconds.
		Weight::from_parts(48_000_000, 8012936)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: OrderBook Orders (r:1 w:1)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrmlTokens Accounts (r:4 w:4)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook AssetPairOrders (r:1 w:1)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8000070), added: 8002545, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:1)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn fill_order_full() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2343`
		//  Estimated: `8030644`
		// Minimum execution time: 91_000 nanoseconds.
		Weight::from_parts(92_000_000, 8030644)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
