
//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 8_700 nanoseconds.
		Weight::from_ref_time(8_900_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 7_800 nanoseconds.
		Weight::from_ref_time(15_251_915 as u64)
			// Standard Error: 4_739
			.saturating_add(Weight::from_ref_time(1_054_768 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_400_000 as u64)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 43_600 nanoseconds.
		Weight::from_ref_time(43_701_000 as u64)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(1_632 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	fn service_task_named() -> Weight {
		// Minimum execution time: 21_900 nanoseconds.
		Weight::from_ref_time(22_301_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 18_900 nanoseconds.
		Weight::from_ref_time(19_201_000 as u64)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 9_300 nanoseconds.
		Weight::from_ref_time(9_500_000 as u64)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 9_200 nanoseconds.
		Weight::from_ref_time(9_400_000 as u64)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(43_464_233 as u64)
			// Standard Error: 9_994
			.saturating_add(Weight::from_ref_time(1_076_173 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 38_900 nanoseconds.
		Weight::from_ref_time(42_002_840 as u64)
			// Standard Error: 7_015
			.saturating_add(Weight::from_ref_time(1_085_938 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 40_401 nanoseconds.
		Weight::from_ref_time(49_499_682 as u64)
			// Standard Error: 10_101
			.saturating_add(Weight::from_ref_time(1_146_865 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 41_301 nanoseconds.
		Weight::from_ref_time(45_488_912 as u64)
			// Standard Error: 5_946
			.saturating_add(Weight::from_ref_time(1_120_735 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
