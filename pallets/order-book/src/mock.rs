// Copyright 2023 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_mocks::pallet_mock_fees;
use cfg_types::tokens::{CurrencyId, CustomMetadata};
use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU32, GenesisBuild},
};
use orml_traits::{asset_registry::AssetMetadata, parameter_type_with_key, GetByKey};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use crate as order_book;

pub(crate) const STARTING_BLOCK: u64 = 50;
pub(crate) const ACCOUNT_0: u64 = 0x1;
pub(crate) const ACCOUNT_1: u64 = 0x2;
pub(crate) const ORDER_FEEKEY: u8 = 0u8;
pub(crate) const ORDER_FEEKEY_AMOUNT: u128 = 10 * CURRENCY_NATIVE;

pub(crate) const CURRENCY_AUSD: Balance = 1_000_000;
// To ensure price/amount calculations with different
// currency precision works
pub(crate) const CURRENCY_FA0: Balance = 1_000;

pub(crate) const CURRENCY_NATIVE: Balance = 1_000_000;

type Balance = u128;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;
pub type MockAccountId = u64;

frame_support::construct_runtime!(
	  pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	  {
			Balances: pallet_balances,
			Fees: pallet_mock_fees,
			System: frame_system,
		  OrmlTokens: orml_tokens,
		  OrderBook: order_book,
			Tokens: pallet_restricted_tokens,
	  }
);

parameter_types! {
	  pub const BlockHashCount: u64 = 250;
	  pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
	type AccountData = pallet_balances::AccountData<Balance>;
	type AccountId = MockAccountId;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

impl pallet_mock_fees::Config for Runtime {
	type Balance = Balance;
	type FeeKey = u8;
}

parameter_types! {
	  pub const DefaultFeeValue: Balance = 1;
}

impl pallet_balances::Config for Runtime {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

cfg_test_utils::mocks::orml_asset_registry::impl_mock_registry! {
		RegistryMock,
		CurrencyId,
		Balance,
		CustomMetadata
}

parameter_type_with_key! {
		pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
				Default::default()
		};
}

impl orml_tokens::Config for Runtime {
	type Amount = i64;
	type Balance = Balance;
	type CurrencyHooks = ();
	type CurrencyId = CurrencyId;
	type DustRemovalWhitelist = frame_support::traits::Nothing;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {

		pub const NativeToken: CurrencyId = CurrencyId::Native;
}

impl pallet_restricted_tokens::Config for Runtime {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type Fungibles = OrmlTokens;
	type NativeFungible = Balances;
	type NativeToken = NativeToken;
	type PreCurrency = cfg_traits::Always;
	type PreExtrTransfer = cfg_traits::Always;
	type PreFungibleInspect = pallet_restricted_tokens::FungibleInspectPassthrough;
	type PreFungibleInspectHold = cfg_traits::Always;
	type PreFungibleMutate = cfg_traits::Always;
	type PreFungibleMutateHold = cfg_traits::Always;
	type PreFungibleTransfer = cfg_traits::Always;
	type PreFungiblesInspect = pallet_restricted_tokens::FungiblesInspectPassthrough;
	type PreFungiblesInspectHold = cfg_traits::Always;
	type PreFungiblesMutate = cfg_traits::Always;
	type PreFungiblesMutateHold = cfg_traits::Always;
	type PreFungiblesTransfer = cfg_traits::Always;
	type PreReservableCurrency = cfg_traits::Always;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
		pub const OrderPairVecSize: u32 = 1_000_000u32;
}

parameter_type_with_key! {
		pub MinimumOrderAmount: |pair: (CurrencyId, CurrencyId)| -> Option<Balance> {
				match pair {
						(CurrencyId::Native, CurrencyId::AUSD) => Some(5 * CURRENCY_NATIVE),
						(CurrencyId::AUSD, CurrencyId::Native) => Some(5 * CURRENCY_AUSD),
						(CurrencyId::AUSD, CurrencyId::ForeignAsset(0)) => Some(5 * CURRENCY_AUSD),
						(CurrencyId::ForeignAsset(0), CurrencyId::AUSD) => Some(5 * CURRENCY_FA0),
						(CurrencyId::Native, CurrencyId::ForeignAsset(0)) => Some(5 * CURRENCY_NATIVE),
						(CurrencyId::ForeignAsset(0), CurrencyId::Native) => Some(5 * CURRENCY_FA0),
						_ => None
				}
		};
}

impl order_book::Config for Runtime {
	type AssetCurrencyId = CurrencyId;
	type AssetRegistry = RegistryMock;
	type Balance = Balance;
	type MinimumOrderAmount = MinimumOrderAmount;
	type OrderIdNonce = u64;
	type OrderPairVecSize = OrderPairVecSize;
	type RuntimeEvent = RuntimeEvent;
	type SellRatio = cfg_types::fixed_point::Rate;
	type TradeableAsset = Tokens;
	type Weights = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	// Add native balances for reserve/unreserve storage fees
	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(ACCOUNT_0, 300 * CURRENCY_NATIVE),
			(ACCOUNT_1, 300 * CURRENCY_NATIVE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	// Add foreign currency balances of differing precisions
	orml_tokens::GenesisConfig::<Runtime> {
		balances: (0..3)
			.into_iter()
			.flat_map(|idx| {
				[
					(idx, CurrencyId::AUSD, 1000 * CURRENCY_AUSD),
					(idx, CurrencyId::ForeignAsset(0), 1000 * CURRENCY_FA0),
					(idx, CurrencyId::Native, 100 * CURRENCY_AUSD),
				]
			})
			.collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	orml_asset_registry_mock::GenesisConfig {
		metadata: vec![
			(
				CurrencyId::AUSD,
				AssetMetadata {
					decimals: 6,
					name: "MOCK TOKEN_A".as_bytes().to_vec(),
					symbol: "MOCK_A".as_bytes().to_vec(),
					existential_deposit: 0,
					location: None,
					additional: CustomMetadata::default(),
				},
			),
			(
				CurrencyId::ForeignAsset(0),
				AssetMetadata {
					decimals: 3,
					name: "MOCK TOKEN_B".as_bytes().to_vec(),
					symbol: "MOCK_B".as_bytes().to_vec(),
					existential_deposit: 0,
					location: None,
					additional: CustomMetadata::default(),
				},
			),
			(
				CurrencyId::Native,
				AssetMetadata {
					decimals: 6,
					name: "NATIVE TOKEN".as_bytes().to_vec(),
					symbol: "NATIVE".as_bytes().to_vec(),
					existential_deposit: 0,
					location: None,
					additional: CustomMetadata::default(),
				},
			),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut e = sp_io::TestExternalities::new(t);

	e.execute_with(|| {
		System::set_block_number(STARTING_BLOCK);
		Fees::mock_fee_value(|key| match key {
			ORDER_FEEKEY => ORDER_FEEKEY_AMOUNT.into(),
			_ => panic!("No valid fee key"),
		});
	});
	e
}
