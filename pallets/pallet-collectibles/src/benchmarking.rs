//! Benchmarking setup for pallet-faterium-polls

use super::*;

#[allow(unused)]
use crate::Pallet as CollectiblesModule;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;

benchmarks! {
	create_collectible {
		let caller: T::AccountId = whitelisted_caller();
	  }: create_collectible(RawOrigin::Signed(caller))

	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let unique_id: [u8; 16] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
		let collectible_id = CollectiblesModule::<T>::mint(&caller, unique_id, Color::Red).unwrap();
		let receiver = account::<T::AccountId>("receiver", 1u32, 2u32);
	  }: transfer(RawOrigin::Signed(caller), receiver.clone(), collectible_id)
	  verify {
		assert_eq!(OwnerOfCollectibles::<T>::get(receiver), vec![unique_id]);
	}

	set_price {
		let caller: T::AccountId = whitelisted_caller();
		let unique_id: [u8; 16] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
		let collectible_id = CollectiblesModule::<T>::mint(&caller, unique_id, Color::Red).unwrap();
		let buyer: T::AccountId = whitelisted_caller();
		let new_price = T::Currency::free_balance(&buyer);
	  }: set_price(RawOrigin::Signed(caller), collectible_id,  Some(new_price))

	impl_benchmark_test_suite!(CollectiblesModule, crate::tests::new_test_ext(), crate::tests::Test);
}
