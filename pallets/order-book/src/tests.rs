use frame_support::{assert_err, assert_noop, assert_ok};

use super::*;
use crate::mock::*;

// Extrinsics tests
#[test]
fn create_order_v1_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(OrderBook::create_order_v1(
			RuntimeOrigin::signed(ACCOUNT_0),
			CurrencyId::A,
			CurrencyId::B,
			100,
			10
		));
		let (order_id, _) = OrderBook::get_account_orders(ACCOUNT_0).unwrap()[0];
		assert_eq!(
			Orders::<Runtime>::get(order_id),
			Ok(Order {
				order_id: order_id,
				placing_account: ACCOUNT_0,
				asset_in_id: CurrencyId::A,
				asset_out_id: CurrencyId::B,
				buy_amount: 100,
				initial_buy_amount: 100,
				price: 10,
				min_fullfillment_amount: 100,
				max_sell_amount: 1000
			})
		);
		assert_eq!(
			UserOrders::<Runtime>::get(ACCOUNT_0, order_id),
			Ok(Order {
				order_id: order_id,
				placing_account: ACCOUNT_0,
				asset_in_id: CurrencyId::A,
				asset_out_id: CurrencyId::B,
				buy_amount: 100,
				initial_buy_amount: 100,
				price: 10,
				min_fullfillment_amount: 100,
				max_sell_amount: 1000
			})
		);
		assert_eq!(
			AssetPairOrders::<Runtime>::get(CurrencyId::A, CurrencyId::B),
			vec![order_id,]
		)
	})
}

#[test]
fn user_cancel_order_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(OrderBook::create_order_v1(
			RuntimeOrigin::signed(ACCOUNT_0),
			CurrencyId::A,
			CurrencyId::B,
			100,
			10
		));
		let (order_id, _) = OrderBook::get_account_orders(ACCOUNT_0).unwrap()[0];
		assert_ok!(OrderBook::user_cancel_order(
			RuntimeOrigin::signed(ACCOUNT_0),
			order_id
		));
		assert_err!(
			Orders::<Runtime>::get(order_id),
			Error::<Runtime>::OrderNotFound
		);

		assert_err!(
			UserOrders::<Runtime>::get(ACCOUNT_0, order_id),
			Error::<Runtime>::OrderNotFound
		);

		assert_eq!(
			AssetPairOrders::<Runtime>::get(CurrencyId::A, CurrencyId::B),
			vec![]
		)
	})
}

// TokenSwaps trait impl tests
#[test]
fn place_order_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(OrderBook::place_order(
			ACCOUNT_0,
			CurrencyId::A,
			CurrencyId::B,
			100,
			10,
			100
		));
		let (order_id, _) = OrderBook::get_account_orders(ACCOUNT_0).unwrap()[0];
		assert_eq!(
			Orders::<Runtime>::get(order_id),
			Ok(Order {
				order_id: order_id,
				placing_account: ACCOUNT_0,
				asset_in_id: CurrencyId::A,
				asset_out_id: CurrencyId::B,
				buy_amount: 100,
				initial_buy_amount: 100,
				price: 10,
				min_fullfillment_amount: 100,
				max_sell_amount: 1000
			})
		);

		assert_eq!(
			UserOrders::<Runtime>::get(ACCOUNT_0, order_id),
			Ok(Order {
				order_id: order_id,
				placing_account: ACCOUNT_0,
				asset_in_id: CurrencyId::A,
				asset_out_id: CurrencyId::B,
				buy_amount: 100,
				initial_buy_amount: 100,
				price: 10,
				min_fullfillment_amount: 100,
				max_sell_amount: 1000
			})
		);

		assert_eq!(
			AssetPairOrders::<Runtime>::get(CurrencyId::A, CurrencyId::B),
			vec![order_id,]
		);

		assert_eq!(
			System::events()[0].event,
			RuntimeEvent::Balances(pallet_balances::Event::Reserved {
				who: ACCOUNT_0,
				amount: 10
			})
		);
		assert_eq!(
			System::events()[1].event,
			RuntimeEvent::OrmlTokens(orml_tokens::Event::Reserved {
				currency_id: CurrencyId::B,
				who: ACCOUNT_0,
				amount: 1000
			})
		);
		assert_eq!(
			System::events()[2].event,
			RuntimeEvent::OrderBook(Event::OrderCreated {
				order_id: order_id,
				creator_account: ACCOUNT_0,
				currency_in: CurrencyId::A,
				currency_out: CurrencyId::B,
				buy_amount: 100,
				min_fullfillment_amount: 100,
				sell_price_limit: 10
			})
		);
	})
}

#[test]
fn ensure_nonce_updates_order_correctly() {
	new_test_ext().execute_with(|| {
		assert_ok!(OrderBook::place_order(
			ACCOUNT_0,
			CurrencyId::A,
			CurrencyId::B,
			100,
			10,
			100
		));
		assert_ok!(OrderBook::place_order(
			ACCOUNT_0,
			CurrencyId::A,
			CurrencyId::B,
			100,
			10,
			100
		));
		let [(order_id_0, _), (order_id_1, _)] = OrderBook::get_account_orders(ACCOUNT_0)
			.unwrap()
			.into_iter()
			.collect::<Vec<_>>()[..] else {panic!("Unexpected order count")};
		assert_ne!(order_id_0, order_id_1)
	})
}

#[test]
fn place_order_requires_non_zero_buy() {
	new_test_ext().execute_with(|| {
		assert_err!(
			OrderBook::place_order(ACCOUNT_0, CurrencyId::A, CurrencyId::B, 0, 10, 100),
			Error::<Runtime>::InvalidBuyAmount
		);
	})
}

#[test]
fn place_order_requires_non_zero_price() {
	new_test_ext().execute_with(|| {
		assert_err!(
			OrderBook::place_order(ACCOUNT_0, CurrencyId::A, CurrencyId::B, 100, 0, 100),
			Error::<Runtime>::InvalidMinPrice
		);
	})
}

#[test]
fn cancel_order_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(OrderBook::place_order(
			ACCOUNT_0,
			CurrencyId::A,
			CurrencyId::B,
			100,
			10,
			100
		));
		let (order_id, _) = OrderBook::get_account_orders(ACCOUNT_0).unwrap()[0];
		assert_ok!(OrderBook::cancel_order(order_id));
		assert_err!(
			Orders::<Runtime>::get(order_id),
			Error::<Runtime>::OrderNotFound
		);

		assert_err!(
			UserOrders::<Runtime>::get(ACCOUNT_0, order_id),
			Error::<Runtime>::OrderNotFound
		);

		assert_eq!(
			AssetPairOrders::<Runtime>::get(CurrencyId::A, CurrencyId::B),
			vec![]
		);
		assert_eq!(
			System::events()[3].event,
			RuntimeEvent::Balances(pallet_balances::Event::Unreserved {
				who: ACCOUNT_0,
				amount: 10
			})
		);
		assert_eq!(
			System::events()[4].event,
			RuntimeEvent::OrmlTokens(orml_tokens::Event::Unreserved {
				currency_id: CurrencyId::B,
				who: ACCOUNT_0,
				amount: 1000
			})
		);
		assert_eq!(
			System::events()[5].event,
			RuntimeEvent::OrderBook(Event::OrderCancelled {
				order_id,
				account: ACCOUNT_0,
			})
		);
	});
}
