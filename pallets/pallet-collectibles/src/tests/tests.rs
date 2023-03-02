use super::*;
use crate::{CollectibleMap, CollectiblesCount, OwnerOfCollectibles};
use pallet_collectibles::{Error, Event};
use system::Origin;

#[test]
fn mint_collectible() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let (collectible_id, color) = CollectiblesModule::gen_unique_id();
		let minter: u64 = 1;
		let mint = CollectiblesModule::mint(&minter, collectible_id, color).unwrap();
		println!("mint: {:?}", mint);
		let count = CollectiblesCount::<Test>::get();
		assert_noop!(
			count.checked_add(U64_MAX).ok_or(Error::<Test>::BoundsOverflow),
			Error::<Test>::BoundsOverflow
		);
		let _collection = 1000;
		// assert_noop!(
		// 	OwnerOfCollectibles::<Test>::try_append(&minter, collection.into()),
		// 	Error::<Test>::MaximumCollectiblesOwned
		// );
		System::assert_has_event(tests::RuntimeEvent::CollectiblesModule(
			Event::<Test>::CollectibleCreated { collectible: collectible_id, owner: minter },
		));
		assert!(CollectibleMap::<Test>::contains_key(collectible_id));
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
			Origin::<Test>::Signed(minter).into(),
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
			CollectiblesModule::transfer(
				Origin::<Test>::Signed(receiver).into(),
				receiver,
				collectible_id
			),
			Error::<Test>::TransferToSelf
		);
		assert_noop!(
			CollectiblesModule::transfer(
				Origin::<Test>::Signed(fake_id).into(),
				receiver,
				collectible_id
			),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn set_price_for_collectible() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let (collectible_id, minter) = create_collectible();
		let receiver = 2;
		let new_price = 1000;
		CollectiblesModule::set_price(
			Origin::<Test>::Signed(minter).into(),
			collectible_id,
			Some(new_price),
		)
		.unwrap();
		System::assert_has_event(tests::RuntimeEvent::CollectiblesModule(
			Event::<Test>::PriceSet { collectible: collectible_id, price: Some(new_price) },
		));
		assert_noop!(
			CollectiblesModule::set_price(
				Origin::<Test>::Signed(receiver).into(),
				collectible_id,
				Some(0),
			),
			Error::<Test>::NotOwner
		);
	})
}
