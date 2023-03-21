use cfg_primitives::CFG;
use cfg_traits::{InterestAccrual, Permissions, PoolBenchmarkHelper};
use cfg_types::{
	adjustments::Adjustment,
	permissions::{PermissionScope, PoolRole, Role},
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::{
	tokens::nonfungibles::{Create, Mutate},
	UnixTime,
};
use frame_system::RawOrigin;
use sp_arithmetic::FixedPointNumber;
use sp_runtime::traits::{Get, One, Zero};
use sp_std::{time::Duration, vec};

use super::{
	pallet::*,
	types::{LoanInfo, MaxBorrowAmount, WriteOffState},
	valuation::{DiscountedCashFlow, ValuationMethod},
};

const OFFSET: Duration = Duration::from_secs(120);
const COLLECION_ID: u16 = 42;
const COLLATERAL_VALUE: u128 = 1_000_000;
const FUNDS: u128 = 1_000_000_000;

type MaxRateCountOf<T> = <<T as Config>::InterestAccrual as InterestAccrual<
	<T as Config>::Rate,
	<T as Config>::Balance,
	Adjustment<<T as Config>::Balance>,
>>::MaxRateCount;

struct Helper<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> Helper<T>
where
	T::Balance: From<u128>,
	T::NonFungible: Create<T::AccountId> + Mutate<T::AccountId>,
	T::CollectionId: From<u16>,
	T::ItemId: From<u16>,
	T::Pool:
		PoolBenchmarkHelper<PoolId = PoolIdOf<T>, AccountId = T::AccountId, Balance = T::Balance>,
{
	#[cfg(test)]
	fn config_mocks() {
		use crate::mock::{MockPermissions, MockPools};

		MockPermissions::mock_add(|_, _, _| Ok(()));
		MockPermissions::mock_has(|_, _, _| true);
		MockPools::mock_pool_exists(|_| true);
		MockPools::mock_account_for(|_| 0);
		MockPools::mock_withdraw(|_, _, _| Ok(()));
		MockPools::mock_deposit(|_, _, _| Ok(()));
		MockPools::mock_benchmark_create_pool(|_, _| {});
		MockPools::mock_benchmark_give_ausd(|_, _| {});
	}

	fn prepare_benchmark() -> PoolIdOf<T> {
		#[cfg(test)]
		Self::config_mocks();

		let pool_id = Default::default();

		let pool_admin = account("pool_admin", 0, 0);
		T::Pool::benchmark_create_pool(pool_id, &pool_admin);

		let loan_admin = account("loan_admin", 0, 0);
		T::Permissions::add(
			PermissionScope::Pool(pool_id),
			loan_admin,
			Role::PoolRole(PoolRole::LoanAdmin),
		)
		.unwrap();

		let borrower = account::<T::AccountId>("borrower", 0, 0);
		T::Pool::benchmark_give_ausd(&borrower, (FUNDS * CFG).into());
		T::NonFungible::create_collection(&COLLECION_ID.into(), &borrower, &borrower).unwrap();
		T::Permissions::add(
			PermissionScope::Pool(pool_id),
			borrower.clone(),
			Role::PoolRole(PoolRole::Borrower),
		)
		.unwrap();

		pool_id
	}

	fn create_loan(pool_id: PoolIdOf<T>, item_id: T::ItemId) -> T::LoanId {
		let borrower = account("borrower", 0, 0);

		let collection_id = COLLECION_ID.into();
		T::NonFungible::mint_into(&collection_id, &item_id, &borrower).unwrap();

		Pallet::<T>::create(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			LoanInfo::new((collection_id, item_id))
				.maturity(T::Time::now() + OFFSET)
				.interest_rate(T::Rate::saturating_from_rational(1, 5000))
				.collateral_value((COLLATERAL_VALUE).into())
				.max_borrow_amount(MaxBorrowAmount::UpToOutstandingDebt {
					advance_rate: T::Rate::one(),
				})
				.valuation_method(ValuationMethod::DiscountedCashFlow(DiscountedCashFlow {
					probability_of_default: T::Rate::zero(),
					loss_given_default: T::Rate::zero(),
					discount_rate: T::Rate::one(),
				})),
		)
		.unwrap();

		LastLoanId::<T>::get(pool_id)
	}

	fn borrow_loan(pool_id: PoolIdOf<T>, loan_id: T::LoanId) {
		let borrower = account("borrower", 0, 0);
		Pallet::<T>::borrow(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			loan_id,
			10.into(),
		)
		.unwrap();
	}

	fn fully_repay_loan(pool_id: PoolIdOf<T>, loan_id: T::LoanId) {
		let borrower = account("borrower", 0, 0);
		Pallet::<T>::repay(
			RawOrigin::Signed(borrower).into(),
			pool_id,
			loan_id,
			COLLATERAL_VALUE.into(),
		)
		.unwrap();
	}

	fn set_policy(pool_id: PoolIdOf<T>) {
		let pool_admin = account::<T::AccountId>("pool_admin", 0, 0);

		Pallet::<T>::update_write_off_policy(
			RawOrigin::Signed(pool_admin).into(),
			pool_id,
			vec![WriteOffState {
				overdue_days: 0,
				percentage: T::Rate::zero(),
				penalty: T::Rate::zero(),
			}]
			.try_into()
			.unwrap(),
		)
		.unwrap();
	}

	fn expire_loan(pool_id: PoolIdOf<T>, loan_id: T::LoanId) {
		Pallet::<T>::expire(pool_id, loan_id).unwrap();
	}

	fn initialize_active_state(n: u32) -> PoolIdOf<T> {
		for i in 1..MaxRateCountOf::<T>::get() {
			// First `i` (i=0) used by the loan's interest rate.
			let rate = T::Rate::saturating_from_rational(i + 1, 5000);
			T::InterestAccrual::reference_yearly_rate(rate).unwrap();
		}

		let pool_id = Self::prepare_benchmark();

		for i in 0..n {
			let item_id = (i as u16).into();
			let loan_id = Self::create_loan(pool_id, item_id);
			Self::borrow_loan(pool_id, loan_id);
		}

		pool_id
	}
}

benchmarks! {
	where_clause {
	where
		T::Balance: From<u128>,
		T::NonFungible: Create<T::AccountId> + Mutate<T::AccountId>,
		T::CollectionId: From<u16>,
		T::ItemId: From<u16>,
		T::Pool: PoolBenchmarkHelper<PoolId = PoolIdOf<T>, AccountId = T::AccountId, Balance = T::Balance>,
	}

	create {
		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::prepare_benchmark();

		let (collection_id, item_id) = (COLLECION_ID.into(), 1.into());
		T::NonFungible::mint_into(&collection_id, &item_id, &borrower).unwrap();

		let loan_info = LoanInfo::new((collection_id, item_id)).maturity(T::Time::now() + OFFSET);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_info)

	borrow {
		let n in 1..T::MaxActiveLoansPerPool::get() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id, 10.into())

	repay {
		let n in 1..T::MaxActiveLoansPerPool::get() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id, 10.into())

	write_off {
		let n in 1..T::MaxActiveLoansPerPool::get() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);
		Helper::<T>::set_policy(pool_id);
		Helper::<T>::expire_loan(pool_id, loan_id);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id)

	admin_write_off {
		let n in 1..T::MaxActiveLoansPerPool::get() - 1;

		let loan_admin = account("loan_admin", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);

	}: _(RawOrigin::Signed(loan_admin), pool_id, loan_id, T::Rate::zero(), T::Rate::zero())

	close {
		let n in 1..T::MaxActiveLoansPerPool::get() - 1;

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);
		let loan_id = Helper::<T>::create_loan(pool_id, u16::MAX.into());
		Helper::<T>::borrow_loan(pool_id, loan_id);
		Helper::<T>::fully_repay_loan(pool_id, loan_id);

	}: _(RawOrigin::Signed(borrower), pool_id, loan_id)

	update_write_off_policy {
		let pool_admin = account("pool_admin", 0, 0);
		let pool_id = Helper::<T>::prepare_benchmark();

		let state = WriteOffState {
			overdue_days: 0,
			percentage: T::Rate::zero(),
			penalty: T::Rate::zero(),
		};
		let policy = vec![state; T::MaxWriteOffPolicySize::get() as usize]
			.try_into()
			.unwrap();

	}: _(RawOrigin::Signed(pool_admin), pool_id, policy)

	update_portfolio_valuation {
		let n in 1..T::MaxActiveLoansPerPool::get();

		let borrower = account("borrower", 0, 0);
		let pool_id = Helper::<T>::initialize_active_state(n);

	}: _(RawOrigin::Signed(borrower), pool_id)
	verify {
		assert!(Pallet::<T>::portfolio_valuation(pool_id).value() > Zero::zero());
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Runtime);
