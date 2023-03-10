use frame_system::ensure_signed;
use hex::FromHex;
use pallet_connectors::DomainAddress;
use sp_core::H160;
use xcm::{v1::MultiLocation, VersionedMultiLocation};

use super::*;
use crate::mock::*;

const SENDER: u64 = 0x1;

#[test]
fn from_account_works() {
	new_test_ext().execute_with(|| {
		let a = ensure_signed(RuntimeOrigin::signed(SENDER)).unwrap();
		let l: Location<Runtime> = Location::<Runtime>::from(AccountWrapper(a));
		assert_eq!(l, Location::Local(a))
	});
}
#[test]
fn from_xcm_address_works() {
	new_test_ext().execute_with(|| {
		let xa = MultiLocation::default();
		let l = Location::<Runtime>::from(xa.clone());
		assert_eq!(l, Location::XCMV1(MultiLocation::default()))
	});
}
#[test]
fn from_domain_address_works() {
	new_test_ext().execute_with(|| {
		let da = DomainAddress::EVM(
			1284,
			<[u8; 20]>::from_hex("1231231231231231231231231231231231231231").expect(""),
		);
		let l = Location::<Runtime>::from(da.clone());

		assert_eq!(l, Location::Address(da))
	});
}
