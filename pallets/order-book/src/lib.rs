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

// This pallet was made using the ZeitGeist Orderbook pallet as a reference;
// with much of the code being copied or adapted from that pallet.
// The ZeitGeist Orderbook pallet can be found here: https://github.com/zeitgeistpm/zeitgeist/tree/main/zrml/orderbook-v1

#![cfg_attr(not(feature = "std"), no_std)]

//! This module adds an orderbook pallet, allowing oders for currency swaps to
//! be placed and fulfilled for currencies in an asset registry.

#[cfg(test)]
pub(crate) mod mock;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {

	use core::fmt::Debug;

	use cfg_traits::fees::Fees;
	use cfg_types::tokens::{CustomMetadata, GeneralCurrencyIndex};
	use codec::{Decode, Encode, MaxEncodedLen};
	use frame_support::{
		pallet_prelude::{
			DispatchResult, Member, OptionQuery, StorageDoubleMap, StorageNMap, StorageValue, *,
		},
		traits::{tokens::AssetId, Currency, ReservableCurrency},
		Twox64Concat,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use orml_traits::{
		asset_registry::{self, Inspect as _},
		MultiCurrency, MultiReservableCurrency,
	};
	use scale_info::TypeInfo;
	use sp_runtime::traits::{AtLeast32BitUnsigned, EnsureAdd, EnsureMul, EnsureSub, Hash, One};

	use super::*;

	/// Balance type for the reserve/deposit made when creating an Allowance
	pub type DepositBalanceOf<T> = <<T as Config>::ReserveCurrency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;
	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]

	pub struct Pallet<T>(_);
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type AssetRegistry: asset_registry::Inspect<
			AssetId = Self::AssetCurrencyId,
			Balance = Self::Balance,
			CustomMetadata = CustomMetadata,
		>;

		/// CurrencyId of Asset
		type AssetCurrencyId: AssetId
			+ Parameter
			+ Debug
			+ Default
			+ Member
			+ Copy
			+ MaybeSerializeDeserialize
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;

		/// Currency for Reserve/Unreserve with allowlist adding/removal,
		/// given that the allowlist will be in storage
		type ReserveCurrency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Fee handler for the reserve/unreserve
		/// Currently just stores the amounts, will be extended to handle
		/// reserve/unreserve as well in future
		type Fees: Fees<
			AccountId = <Self as frame_system::Config>::AccountId,
			Balance = DepositBalanceOf<Self>,
		>;

		/// Fee Key used to find amount for allowance reserve/unreserve
		type OrdeFeeKey: Get<<Self::Fees as Fees>::FeeKey>;

		type Balance: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ EnsureAdd
			+ EnsureSub
			+ EnsureMul
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;

		type Nonce: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ EnsureAdd
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;

		/// Type for trade-able currency
		type TradeableAsset: MultiReservableCurrency<
			Self::AccountId,
			Balance = <Self as pallet::Config>::Balance,
			CurrencyId = Self::AssetCurrencyId,
		>;
	}
	//
	// Storage and storage types
	//
	#[derive(Clone, Copy, Debug, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	pub struct Order<OrderId, AccountId, AssetId, Balance> {
		order_id: OrderId,
		placing_account: AccountId,
		asset_in_id: AssetId,
		asset_out_id: AssetId,
		sell_amount: Balance,
		price: Balance,
	}

	#[derive(Clone, Copy, Debug, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
	pub struct Claim<T: Config> {
		claiming_account: T::AccountId,
		order_claiming: T::Hash,
	}

	#[pallet::storage]
	pub type Orders<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::Hash,
		Order<T::Hash, T::AccountId, T::AssetCurrencyId, T::Balance>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type UserOrders<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::AccountId,
		Twox64Concat,
		T::Hash,
		Order<T::Hash, T::AccountId, T::AssetCurrencyId, T::Balance>,
		OptionQuery,
	>;

	/// Stores Nonce for orders placed
	/// Given that Nonce is to ensure that all orders have a unique ID, we can
	/// use just one Nonce, which means that we only have one val in storage,
	/// and we don't have to insert new map values upon a new account/currency
	/// order creation.
	#[pallet::storage]
	pub type NonceStore<T: Config> = StorageValue<_, T::Nonce, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		InvalidAssetId,
		ConflictingAssetIds,
		InsufficientAssetFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		// dummy weight for now
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn create_order(
			origin: OriginFor<T>,
			asset_in: T::AssetCurrencyId,
			asset_out: T::AssetCurrencyId,
			amount: T::Balance,
			price: T::Balance,
		) -> DispatchResult {
			let account_id = ensure_signed(origin)?;
			ensure!(asset_in != asset_out, Error::<T>::ConflictingAssetIds);
			ensure!(
				T::AssetRegistry::metadata(&asset_in).is_some(),
				Error::<T>::InvalidAssetId
			);
			ensure!(
				T::AssetRegistry::metadata(&asset_out).is_some(),
				Error::<T>::InvalidAssetId
			);
			ensure!(
				T::TradeableAsset::can_reserve(asset_in, &account_id, amount),
				Error::<T>::InsufficientAssetFunds,
			);
			<NonceStore<T>>::try_mutate(|n| {
				*n = n.ensure_add(T::Nonce::one())?;
				Ok::<_, DispatchError>(())
			})?;
			let new_nonce = <NonceStore<T>>::get();
			let order_id = Self::gen_hash(&account_id, asset_in, asset_out, new_nonce);
			let new_order: Order<T::Hash> = Order {
				order_id: order_id,
				placing_account: account_id.clone(),
				asset_in_id: asset_in,
				asset_out_id: asset_out,
				sell_amount: amount,
				price: price,
			};
			T::TradeableAsset::reserve(asset_in, &account_id, amount)?;
			<Orders<T>>::insert(order_id, new_order);
			<UserOrders<T>>::insert(account_id, order_id, new_order);

			Ok(())
		}

		#[pallet::call_index(1)]
		// dummy weight for now
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn cancel_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResult {
			let account_id = ensure_signed(origin)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		// dummy weight for now
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn fill_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResult {
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn gen_hash(
			placer: &T::AccountId,
			asset_out: T::AssetCurrencyId,
			asset_in: T::AssetCurrencyId,
			nonce: T::Nonce,
		) -> T::Hash {
			(&placer, asset_in, asset_out, nonce).using_encoded(T::Hashing::hash)
		}
	}
}
