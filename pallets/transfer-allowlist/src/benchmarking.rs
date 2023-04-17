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

use cfg_types::{fee_keys::FeeKey, locations::Location, tokens::CurrencyId};
use codec::{EncodeLike, WrapperTypeDecode};
use frame_benchmarking::*;
use frame_support::traits::{tokens::AssetId, ReservableCurrency};
use frame_system::RawOrigin;
use sp_runtime::{traits::AtLeast32BitUnsigned, Saturating};

use super::*;
benchmarks! {
	where_clause {
	  where
		T: Config<CurrencyId = CurrencyId, Location = Location>,
	<T as frame_system::Config>::AccountId: Into<<T as pallet::Config>::Location> + AtLeast32BitUnsigned,
	  <T as pallet::Config>::Location: From<<T as frame_system::Config>::AccountId> + EncodeLike<<T as pallet::Config>::Location>,
	  <T as frame_system::Config>::BlockNumber: AtLeast32BitUnsigned

}

	add_transfer_allowance {
	  let sender: T::AccountId = account::<T::AccountId>("Sender", 1,0);
	  let receiver: T::AccountId = account::<T::AccountId>("Receiver", 2,0);

	}:add_transfer_allowance(RawOrigin::Signed(sender.clone()), CurrencyId::Native, receiver.clone().into())
	verify {
	  assert_eq!(
				Pallet::<T>::get_account_currency_transfer_allowance(
					  (sender,
					  CurrencyId::Native,
					  Location::from(receiver))
				).unwrap(),
				AllowanceDetails {
					  allowed_at: 0u32.into(),
					  blocked_at: u32::MAX.into(),
				}
		  )

  }
  // remove_allowance {
	//   add_transfer_allowance(RuntimeOrigin::signed(SENDER), CurrencyId::A, ACCOUNT_RECIEVER.into())
  // }:remove_transfer_allowance(
	// 		RuntimeOrigin::signed(SENDER),
	// 		CurrencyId::A,
	// 		ACCOUNT_RECEIVER.into(),
	// )
	// verify {

	// 	  assert_eq!(
	// 			TransferAllowList::get_account_currency_transfer_allowance((
	// 				  SENDER,
	// 				  CurrencyId::A,
	// 				  Location::TestLocal(ACCOUNT_RECEIVER)
	// 			))
	// 				.unwrap(),
	// 			AllowanceDetails {
	// 				  // current block is 50, no delay set
	// 				  allowed_at: 0u64,
	// 				  blocked_at: 50u64,
	// 			}
	// 	  );
	// }
	// purge_allowance {

	// }: {}
	// verify {

	// }
	// add_delay {

	// }: {}
	// verify {

	// }

	// update_delay {

	// }: {}
	// verify {

	// }

	// toggle_delay_future_modifiable {

	// }: {}
	// verify {

	// }
	// purge_delay {

	// }: {}
	// verify {

	// }

}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Runtime,);
