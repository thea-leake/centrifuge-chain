
//! Autogenerated weights for `pallet_anchors`
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
// --pallet=pallet_anchors
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_anchors.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_anchors`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_anchors::WeightInfo for WeightInfo<T> {
	// Storage: Anchor AnchorEvictDates (r:1 w:0)
	// Storage: Anchor PreCommits (r:1 w:1)
	// Storage: Fees FeeBalances (r:1 w:0)
	fn pre_commit() -> Weight {
		// Minimum execution time: 62_400 nanoseconds.
		Weight::from_ref_time(63_300_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Anchor AnchorEvictDates (r:1 w:1)
	// Storage: Anchor PreCommits (r:1 w:1)
	// Storage: Fees FeeBalances (r:1 w:0)
	// Storage: Authorship Author (r:1 w:0)
	// Storage: System Digest (r:1 w:0)
	// Storage: Anchor LatestAnchorIndex (r:1 w:1)
	// Storage: Anchor AnchorIndexes (r:0 w:1)
	// Storage: unknown [0xdb4faa73ca6d2016e53c7156087c176b79b169c409b8a0063a07964f3187f9e9] (r:0 w:1)
	fn commit() -> Weight {
		// Minimum execution time: 94_600 nanoseconds.
		Weight::from_ref_time(95_800_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Anchor PreCommits (r:100 w:100)
	fn evict_pre_commits() -> Weight {
		// Minimum execution time: 2_026_199 nanoseconds.
		Weight::from_ref_time(2_040_498_000 as u64)
			.saturating_add(T::DbWeight::get().reads(100 as u64))
			.saturating_add(T::DbWeight::get().writes(100 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Anchor LatestEvictedDate (r:1 w:1)
	// Storage: Anchor EvictedAnchorRoots (r:100 w:100)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72010000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72020000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72030000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72040000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72050000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72060000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72070000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72080000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72090000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f720a0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f720b0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f720c0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f720d0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f720e0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f720f0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72100000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72110000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72120000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72130000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72140000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72150000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72160000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72170000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72180000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72190000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f721a0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f721b0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f721c0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f721d0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f721e0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f721f0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72200000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72210000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72220000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72230000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72240000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72250000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72260000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72270000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72280000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72290000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f722a0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f722b0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f722c0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f722d0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f722e0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f722f0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72300000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72310000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72320000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72330000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72340000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72350000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72360000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72370000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72380000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72390000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f723a0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f723b0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f723c0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f723d0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f723e0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f723f0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72400000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72410000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72420000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72430000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72440000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72450000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72460000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72470000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72480000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72490000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f724a0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f724b0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f724c0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f724d0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f724e0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f724f0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72500000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72510000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72520000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72530000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72540000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72550000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72560000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72570000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72580000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72590000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f725a0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f725b0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f725c0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f725d0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f725e0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f725f0000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72600000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72610000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72620000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72630000] (r:1 w:0)
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a616e63686f72640000] (r:1 w:0)
	// Storage: Anchor LatestEvictedAnchorIndex (r:1 w:1)
	// Storage: Anchor LatestAnchorIndex (r:1 w:0)
	// Storage: Anchor AnchorIndexes (r:100 w:100)
	// Storage: Anchor AnchorEvictDates (r:100 w:100)
	// Storage: unknown [0x01d5998dcaa249dfa2a455ae4c045d761623f268227068931dbabca3732aa41f] (r:0 w:1)
	// Storage: unknown [0x04575ee0699f1fa86cccfdcf4285aa81b9bfa0f8837cf533346d722970f1a704] (r:0 w:1)
	// Storage: unknown [0x0959721f200e92d5090cee3c2c4546c11f9bfd16ded1e70e6781d2402880f1f3] (r:0 w:1)
	// Storage: unknown [0x0a958b15afac1ffb0c6e73c553bd8b4ba94ad2d0cc118dcd2a7bc8802e2e772a] (r:0 w:1)
	// Storage: unknown [0x0c4c531cd9dcf8573a6350d0ac9fb060d273156bdee4fdae0043b6fee5bda27c] (r:0 w:1)
	// Storage: unknown [0x0cd3f3ee9420f9c3b2e70862996e8d02e87d1f148632a36b8f72c9548b10b856] (r:0 w:1)
	// Storage: unknown [0x10876da12e1227a2c04872ce311f768aaf3e21458e6ad1c04f044c97fe8e214e] (r:0 w:1)
	// Storage: unknown [0x10b360a66313de6ab2d43019c5fd7ea0db088efb3e1d4a24d89775e66e089cff] (r:0 w:1)
	// Storage: unknown [0x16d33ce142442dfbe857e2c9e0648d026c6bb367d467d6922c2c1133aaa3d7b8] (r:0 w:1)
	// Storage: unknown [0x16e133fb9e42d5a2a9a2e21b2e0efd735fccb527162a21cf520c3aecd84c89ed] (r:0 w:1)
	// Storage: unknown [0x16fcb5e799a48fa04deaaaa71c85bc8e9126bd4b5dbcb3a1f8068ab14bc1c26f] (r:0 w:1)
	// Storage: unknown [0x1b3289127bc95ed117e77d479ccd3ac4477ef8d32df7265bbd42c75bf1945464] (r:0 w:1)
	// Storage: unknown [0x1ecb14235f21b57f49e32ac4f35a1af6a71f96867f0bc61bc5905b8d437b6bde] (r:0 w:1)
	// Storage: unknown [0x1f8b0dafc67f9d378cf0596c5d49f220e5880b9c74ccaadac2206a35ec92715a] (r:0 w:1)
	// Storage: unknown [0x24a8d9c362d9365f46f899adb37f6b61134dceaa80f96a9cda6b059a1301f380] (r:0 w:1)
	// Storage: unknown [0x2a00fca93dceceb635a80a95e8f785b189a4ce35f90a17acba5d1bcacf895a84] (r:0 w:1)
	// Storage: unknown [0x2b318def38ef5f2f8db787e365834ece79fbde70c22cf7bd6c9326995fd4c07f] (r:0 w:1)
	// Storage: unknown [0x2fbeff7b90831a847716e729a30f028899726193b4406a1c91fce4e97beb61b5] (r:0 w:1)
	// Storage: unknown [0x30dc983a9ad263028d0e91a8a0cf703a2a7fd3834b1102f1ff3f8c8876a207bf] (r:0 w:1)
	// Storage: unknown [0x3187d0cdac28db7ec343a07f0b2e44fc56986f0a9c2062d5fa60f99419707bea] (r:0 w:1)
	// Storage: unknown [0x3596cd6b45e209629c71765c804f324ed440f7a1cb2ff6cb542156fd5d213de2] (r:0 w:1)
	// Storage: unknown [0x3645890bd8ab0cc13921468d56eee7da40fbe28dc05bc30a64f05a2c03a1912e] (r:0 w:1)
	// Storage: unknown [0x384b604969634cf37d988e886b5267a51baeb797e09a1d1a0893e5be8fc553df] (r:0 w:1)
	// Storage: unknown [0x3c056a888ea28c9294c91723916f5891141a824048335e32532e6605ce0457e0] (r:0 w:1)
	// Storage: unknown [0x3c5fd1d5c95885c6b44e0f3995886046d906821de1ed5ee95b51b17c42d3295b] (r:0 w:1)
	// Storage: unknown [0x3e74dfe3befcf6fa20eb902c2007ba7fd831619013aa99e016284597b896115b] (r:0 w:1)
	// Storage: unknown [0x42f1cff854d41b18ae379b012a1e712f036bcd839244d5c6324f12c28f6fd6e9] (r:0 w:1)
	// Storage: unknown [0x457803d743c32f50866dbf7aabb339a1d8b6b759783b0627128f0cfd3d6c8775] (r:0 w:1)
	// Storage: unknown [0x4cb17fd2f1d1b2eff69f0ffa1a97ff13e7bf4f05a7a99dd06e503e7546b23906] (r:0 w:1)
	// Storage: unknown [0x58357c4f5a9881658ffc42faa5f48e2810169bf85c8c78011696a17b59728ef5] (r:0 w:1)
	// Storage: unknown [0x5baa983aa91ad92c66e17d16e0757ec4a67ec2ce5b95f4d02ec22fba0e485da0] (r:0 w:1)
	// Storage: unknown [0x5da83d0712f41714545470b781e0a43c65a0ac977327475baa98b5cd94938f17] (r:0 w:1)
	// Storage: unknown [0x6365aeecd6b54d3166f3df46d8c7b404711ca54b4284e8faf67eb014fa3685f8] (r:0 w:1)
	// Storage: unknown [0x683b74d821a8019cbfc9dbe47b50b0f377e0eef16dbc52f7f931ae713fd3f644] (r:0 w:1)
	// Storage: unknown [0x6b02568ad8557dc3d66463abfd1d7f298a0b314fe4bf7d5be79b66768096ed90] (r:0 w:1)
	// Storage: unknown [0x6b05c068aecc171915a61cf59146e7f9a69b9bba39f4df50cecfeb454850b4c9] (r:0 w:1)
	// Storage: unknown [0x6b5529ac614dcbd6113176256a4f5809eb667bddab2e22579306de0a1f83f287] (r:0 w:1)
	// Storage: unknown [0x6cd1381490331969f37f1e6575081f42f1bd8ae0cc79d70fc52ed178b5d75bd0] (r:0 w:1)
	// Storage: unknown [0x6f5b021a9f57d7669ed7269e7d8785acf255f15785bf452a03a4decc184fd403] (r:0 w:1)
	// Storage: unknown [0x764bac7888f79c071087d351a356a09cb2490cb6ea6d71f0cd391de89a885cd2] (r:0 w:1)
	// Storage: unknown [0x7aedb653a5de5739b9d3594196693fd51653fcd59b442e0eb9f64265db188044] (r:0 w:1)
	// Storage: unknown [0x7ca04bdeb932896fd908eb86d4136e9e2462575ebdf981001c1cd3ca6a2faaec] (r:0 w:1)
	// Storage: unknown [0x7ceee738f5af899bd2f967a928019e4a0ecb8715509668dcc039badfe148b45e] (r:0 w:1)
	// Storage: unknown [0x7e700ce9c411e35485babec60c2b68f40c512bc8399c5cee0c1e4264e63f36d1] (r:0 w:1)
	// Storage: unknown [0x80c020f2e70a170ee2f34af3daeda4c2097d14a35f5b1f2d23c2287e5e930f55] (r:0 w:1)
	// Storage: unknown [0x8101d04cf92ee55f6c2a798c7b16da4cc8c511fd822b13093d0f53f5523718d0] (r:0 w:1)
	// Storage: unknown [0x85172de32d6b5871235d50648541b1bd007807512231f9b81f25cb5e20141820] (r:0 w:1)
	// Storage: unknown [0x85e9ccd05d28607dcce0dc5be4f34a7d56d3b83b6c63162b2787fc0e6decf2a7] (r:0 w:1)
	// Storage: unknown [0x87b3d065618080e576b534cf68b60d09c4cca0b71a8b6321337cc23be47e7329] (r:0 w:1)
	// Storage: unknown [0x892ec564231143cc6294a8750b924df2207d91ea3508501d2bd84bee7947b9d0] (r:0 w:1)
	// Storage: unknown [0x8980988eacf42b40c4fc8aa995ae2e059a66c6935626c3e30f1d6842335368d0] (r:0 w:1)
	// Storage: unknown [0x8db2380506697daa88c7a72906d747535ffb12c0ca2a4a6443074bb0fdd8f256] (r:0 w:1)
	// Storage: unknown [0x8e098b9b896a97df275aba887f591c3076220e02adf682c98808e4ba53e6a773] (r:0 w:1)
	// Storage: unknown [0x8e590007efc113bc10a61c478d26803cdae5572d4c70547b3c9813b3ce396826] (r:0 w:1)
	// Storage: unknown [0x96e31df89b1f00b96c993bd9de31e32e7e59c0a185cd0b31adc4e969746c8ea6] (r:0 w:1)
	// Storage: unknown [0x9ae7305289647b636a8702b2316e5482f1a807fa398687068fb653527368f9bc] (r:0 w:1)
	// Storage: unknown [0x9b9660b6fc1992a09573eaa9110c4a08d40c1f439304a47b9776645bc278fc75] (r:0 w:1)
	// Storage: unknown [0xa04f2ef3bb509dfec9d7a97c4778ab2e477af9c5cbda3a1c6e57514314a3f9a5] (r:0 w:1)
	// Storage: unknown [0xa16d64c1e08b47144c2c8e37872486cf440dda823e2ea05f480fedfe83060f17] (r:0 w:1)
	// Storage: unknown [0xa4ad0a32c2781a59ea8a6d58e26fa7dc0b2a08f8c4c938661f5f3ccd8f8eb8ce] (r:0 w:1)
	// Storage: unknown [0xab9797fb6926376ee3b6be73e5501e0a3af18d0bc6dfca0d3b5f498602016956] (r:0 w:1)
	// Storage: unknown [0xac4d9f6628449fe129d24b384441fdb445962d2d6bca7603fea0c20f3d04351c] (r:0 w:1)
	// Storage: unknown [0xafecb421bedaa0f8bd89ef18897b77ce61738af42f8a66e3257a079a3d04bef1] (r:0 w:1)
	// Storage: unknown [0xb292dc48cc1057cce335f1d84f295271a2b16aee7018f1bd444febd77f7e5cbb] (r:0 w:1)
	// Storage: unknown [0xb48b9d9955158dbd87abb433511a5968c21cf78f8085088407e24d6ee26f7f56] (r:0 w:1)
	// Storage: unknown [0xb5a7df612d6fb3bc16c1716414897ba5928835d883003371f02106d5a92abd78] (r:0 w:1)
	// Storage: unknown [0xb684abf2ee5018a16a8dbef6633bcb94a07a2cdf4a173e4fec130da86e8ab987] (r:0 w:1)
	// Storage: unknown [0xb86c8391d2a3eb28b9e3b603cf6929849d50e439e0bbc79781b2555f9cbaa013] (r:0 w:1)
	// Storage: unknown [0xba070ba6cf5f2489f98b6841d238eee4fc403d3065b57f9e3e38ca540971024d] (r:0 w:1)
	// Storage: unknown [0xbcb96e5fc092d3ac258a81b5390671817730859598470874ef02f998518bbf58] (r:0 w:1)
	// Storage: unknown [0xc008db6f6d721d80fab2eab8b6dda4f19bd5def30aa7db86dadd6eb799c2f5ad] (r:0 w:1)
	// Storage: unknown [0xc054c4045e44e28cef1884c0aa86d0049b76eaff493a6d694394df7b0cee8136] (r:0 w:1)
	// Storage: unknown [0xc315216d50f4dd95914d6d102976dc09ec4474da5c314a15f09972ded6e71ddb] (r:0 w:1)
	// Storage: unknown [0xc4a2c3fa3cc7ed1611651510eb6e225abab30676f0fad28c115482c7dd61f8e0] (r:0 w:1)
	// Storage: unknown [0xc6cc01d59d3c86a1c12a167e149d784295fcd13862e4afb0a39a8459e6e25561] (r:0 w:1)
	// Storage: unknown [0xc712d8fa08dd521e5f901ca6d36134807c5ec0510e3b52e8ae5a15f7c13d2ebd] (r:0 w:1)
	// Storage: unknown [0xc7e2bc91ff1b307f6995683b76f1904ccdada3cf8f00528c08d4f65911c4888a] (r:0 w:1)
	// Storage: unknown [0xccbca45304d59a1167eaf9b459e09cffce3d90c087ee9edf8e7e2dc40349373b] (r:0 w:1)
	// Storage: unknown [0xccc17a821dda11e5239ea8dbedee5bd6622fc8dd63ee229fc3bd2dead22e8ae2] (r:0 w:1)
	// Storage: unknown [0xccee04c4c0534d4245892ed24d7814cd14a41aeed7e94591354315f5b74d89f5] (r:0 w:1)
	// Storage: unknown [0xcf67e9890d936f6bd205710c9a5cedc653d88fba3c74b7a2b9fe8ce7fce0bd0c] (r:0 w:1)
	// Storage: unknown [0xcfdb7c67ada01beee8308b04c3f32e4c078603d0c84c0e28e605a8ea56dcc362] (r:0 w:1)
	// Storage: unknown [0xd0d54b0c405fea6ff90809070bfd270c88e9a26ad83138eeb077d8f9602670bc] (r:0 w:1)
	// Storage: unknown [0xd1d4eefa482f2ece90773426cd76c1da272ef0e72c1172a4a71b84c1f5f6c7c7] (r:0 w:1)
	// Storage: unknown [0xd282fcd4ae056e61acbc8950a306910569f227182c41e5b88159aed160ba2a58] (r:0 w:1)
	// Storage: unknown [0xd37f5ea81d5d617ed7490c928e4f3a1eba6f234787ba84f31e204e8733cd039f] (r:0 w:1)
	// Storage: unknown [0xd6780cc86f71e3b9d0f0f6977d180e26166b517ee3ee227701f9f36cccae3171] (r:0 w:1)
	// Storage: unknown [0xd79237f18c61e22111652b0e9b809fbe8ca41552b3a927877a294a732b338f63] (r:0 w:1)
	// Storage: unknown [0xd8825b3a03921d36a1543c344d9b3cacce95765f29c735cf3ed72dc9c37ff81b] (r:0 w:1)
	// Storage: unknown [0xdd012b8629cc16d3ad36b73df7dd7d38e8c11ac479b99dedffb10b5007c8049a] (r:0 w:1)
	// Storage: unknown [0xdec56d85d6fffd793180a2ce033397f67fb3b9b7ac3e2b0ef6be2f15e7de435f] (r:0 w:1)
	// Storage: unknown [0xe1f270fea944a3a9db5550d742e3acb3dd449cafb73dce65c1705d0752c1343b] (r:0 w:1)
	// Storage: unknown [0xe4002351550f1b106219729b86aa4776fb907737c9cd7e957c5ce80062a8ff8a] (r:0 w:1)
	// Storage: unknown [0xe45f26671be0fb4144ed09c40b9493c4584affb2c1d1fe6cb067aa2df802027e] (r:0 w:1)
	// Storage: unknown [0xe6b4a4991b976360dacf2c942d16326dd53584aca6ed1ae4e78f668d7b1163c1] (r:0 w:1)
	// Storage: unknown [0xe8150db238f56576dcf5e1b98f3915361092aa174b16e6cda3e78c28b6444dc8] (r:0 w:1)
	// Storage: unknown [0xebc5f1d9670cdeb0655d79e95c9602ec1d85ad989ce78194dfd1a31e9fb4994c] (r:0 w:1)
	// Storage: unknown [0xed0df01311d268fc75f0da4859b6508e1c445e713847efbc18528d731316cf48] (r:0 w:1)
	// Storage: unknown [0xee60c64e1e32117f948ee71d391f978e8ac98c2bd869322fc25164502e3f7a9b] (r:0 w:1)
	// Storage: unknown [0xf7e4b8a5415405a940e730546df85583c8c23956d99a3be18e09eebf3639d312] (r:0 w:1)
	fn evict_anchors() -> Weight {
		// Minimum execution time: 2_249_899 nanoseconds.
		Weight::from_ref_time(2_285_798_000 as u64)
			.saturating_add(T::DbWeight::get().reads(404 as u64))
			.saturating_add(T::DbWeight::get().writes(402 as u64))
	}
}
