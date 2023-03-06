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
#![cfg_attr(not(feature = "std"), no_std)]

use cfg_primitives::AccountId;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::{DispatchError, DispatchResult};
pub use pallet::*;
use pallet_connectors::DomainAddress;
use scale_info::TypeInfo;
use sp_core::H160;
use sp_runtime::AccountId32;
use xcm::v1::MultiLocation;

#[derive(Clone, Encode, Debug, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum Location {
	Local(AccountId32),
	// unfortunately VersionedMultiLocation does not implmenent MaxEncodedLen, and
	// both are foreign, and therefore can't be implemented here.
	// may move back to new type off VersionedMultiLocation w/ MaxEncodedLen implemented
	// if it looks like nothing will be Location enum outside of pallet
	XCMV1(MultiLocation),
	Address(DomainAddress),
}

impl From<AccountId32> for Location {
	fn from(a: AccountId32) -> Location {
		Self::Local(a)
	}
}

// using
impl From<MultiLocation> for Location {
	fn from(ml: MultiLocation) -> Location {
		Self::XCMV1(ml)
	}
}

impl From<DomainAddress> for Location {
	fn from(da: DomainAddress) -> Location {
		Self::Address(da)
	}
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{
			DispatchResult, OptionQuery, StorageDoubleMap, StorageNMap, ValueQuery, *,
		},
		Twox64Concat,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AtLeast32BitUnsigned;
	use xcm::{v1::MultiLocation, VersionedMultiLocation};

	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type CurrencyIdOf<T> = <T as Config>::CurrencyId;
	pub type AllowanceDetailsOf<T> = AllowanceDetails<BlockNumberOf<T>>;

	#[derive(
		Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo,
	)]
	pub struct AllowanceDetails<BlockNumber> {
		allowed_at: BlockNumber,
		blocked_at: BlockNumber,
	}

	impl<BlockNumber> AllowanceDetails<BlockNumber>
	where
		BlockNumber: AtLeast32BitUnsigned,
	{
		fn default() -> Self {
			Self {
				allowed_at: BlockNumber::zero(),
				blocked_at: BlockNumber::max_value(),
			}
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The currency-id type of this pallet
		type CurrencyId: Parameter
			+ Member
			+ Copy
			+ MaybeSerializeDeserialize
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;
	}

	/// Trait to determine whether a sending account and currency have a restriction,
	/// and if so is there an allowance for the reciever location.
	trait TransferAllowance<AccountId, Location> {
		type CurrencyId;
		fn allowance(
			send: AccountId,
			recieve: Location,
			currency: Self::CurrencyId,
		) -> DispatchResult;
	}

	impl<T: Config> TransferAllowance<Self::AccountId, Self::AccountId> for Pallet<T> {
		type CurrencyId = Self::CurrencyId;

	// 	fn allowance(
	// 		send: Self::AccountId,
	// 		recieve: VersionedMultiLocation,
	// 		currency: Self::Currency,
	// 	) -> DispatchResult {
	// 		if <AccountCurrencyTransferRestriction<T>>::get(send, currency) {
  //         match <AccountCurr>
	// 		} else {
	// 			Ok(())
	// 		}
	// 	}
	// }

	/// Default value for whether an account and currency have transfer restrictions
	#[pallet::type_value]
	pub fn DefaultHasRestrictions<T: Config>() -> bool {
		false
	}
	/// Storage item for whether a sending account and currency have restrictions set
	/// a double map is used here as we need to know whether there is a restriction set
	/// for the account and currency.
	/// Using an StorageNMap would not allow us to look up whether there was a restriction for the sending account and currency, given that:
	/// - we're checking whether there's an allowance specified for the receiver location
	///   - we would only find whether a restriction was set for the account in this caseif:
	///     - an allowance was specified for the receiving location, which would render blocked restrictions useless
	/// - we would otherwise need to store a vec of locations, which is problematic given that there isn't a set limit on receivers
	/// If a transfer restriction is in place, then a second lookup is done on
	/// AccountCurrencyAllowances to see if there is an allowance for the reciever
	/// This allows us to keep storage map vals to known/bounded sizes.
	#[pallet::storage]
	pub type AccountCurrencyTransferRestriction<T> = StorageDoubleMap<
		_,
		Twox64Concat,
		AccountIdOf<T>,
		Twox64Concat,
		CurrencyIdOf<T>,
		bool,
		ValueQuery,
		DefaultHasRestrictions<T>,
	>;

	/// Storage item for allowances specified for a sending account, currency type and drecieving location
	#[pallet::storage]
	pub type AccountCurrencyAllowances<T> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, AccountIdOf<T>>,
			NMapKey<Twox64Concat, CurrencyIdOf<T>>,
			NMapKey<Blake2_128Concat, Location>,
		),
		AllowanceDetails<BlockNumberOf<T>>,
		OptionQuery,
	>;
}

#[cfg(test)]
mod test {
	use cfg_primitives::AccountId;
	use hex::FromHex;
	use pallet_connectors::DomainAddress;
	use sp_core::H160;
	use xcm::{v1::MultiLocation, VersionedMultiLocation};

	use super::*;

	#[test]
	fn from_account_works() {
		let a: AccountId = AccountId::new([0; 32]);
		let l = Location::from(a.clone());
		assert_eq!(l, Location::Local(a))
	}
	#[test]
	fn from_xcm_address_works() {
		let xa = MultiLocation::default();
		let l = Location::from(xa.clone());
		assert_eq!(l, Location::XCMV1(MultiLocation::default()))
	}
	#[test]
	fn from_domain_address_works() {
		let da = DomainAddress::EVM(
			1284,
			<[u8; 20]>::from_hex("1231231231231231231231231231231231231231").expect(""),
		);
		let l = Location::from(da.clone());
		assert_eq!(l, Location::Address(da))
	}
}
