use super::*;
use pallet_collectibles::{Error, Event};

#[test]
fn mint_collectible() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let (collectible_id, color) = CollectiblesModule::gen_unique_id();

		let minter: u64 = 1;
		let mint = CollectiblesModule::mint(&minter, collectible_id, color).unwrap();
		println!("mint: {:?}", mint);

		let count = crate::CollectiblesCount::<Test>::get();
		// Ensure it will panic if count of collectibles is more than U64::MAX
		assert_noop!(
			count.checked_add(U64_MAX).ok_or(Error::<Test>::BoundsOverflow),
			Error::<Test>::BoundsOverflow
		);
		System::assert_has_event(tests::RuntimeEvent::CollectiblesModule(
			Event::<Test>::CollectibleCreated { collectible: collectible_id, owner: minter },
		));
		// Ensure CollectibleMap was append with new generated collectible
		assert!(crate::CollectibleMap::<Test>::contains_key(collectible_id));
	})
}

#[test]
fn transfer_collectible() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let (collectible_id, minter) = create_collectible();
		let receiver: u64 = 2;
		let fake_id = 3;
		assert_ok!(CollectiblesModule::transfer(
			origin_for(minter).into(),
			receiver,
			collectible_id
		));
		System::assert_has_event(tests::RuntimeEvent::CollectiblesModule(
			Event::<Test>::TransferSucceeded {
				from: minter,
				to: receiver,
				collectible: collectible_id,
			},
		));
		assert_noop!(
			CollectiblesModule::transfer(origin_for(receiver).into(), receiver, collectible_id),
			Error::<Test>::TransferToSelf
		);
		assert_noop!(
			CollectiblesModule::transfer(origin_for(fake_id).into(), receiver, collectible_id),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn set_price_and_buy_collectible() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let (collectible_id, minter) = create_collectible();
		let receiver = 2;
		let new_price = 100;
		CollectiblesModule::set_price(origin_for(minter).into(), collectible_id, Some(new_price))
			.unwrap();
		System::assert_has_event(tests::RuntimeEvent::CollectiblesModule(
			Event::<Test>::PriceSet { collectible: collectible_id, price: Some(new_price) },
		));
		assert_noop!(
			CollectiblesModule::set_price(
				origin_for(receiver).into(),
				collectible_id,
				Some(new_price),
			),
			Error::<Test>::NotOwner
		);
		System::reset_events();
		System::set_block_number(2);
		let _selected_price: u64 = 101;
		assert_noop!(
			CollectiblesModule::buy_collectible(origin_for(receiver).into(), collectible_id, 0,),
			Error::<Test>::BidPriceTooLow
		);
	})
}
