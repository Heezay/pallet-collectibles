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
		assert_eq!(OwnerOfCollectibles::<T>::get(receiver), [unique_id].to_vec());
	}

	buy_collectible {
        // Create new collectible 
		let caller: T::AccountId = whitelisted_caller();
		let unique_id: [u8; 16] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
		let collectible_id = CollectiblesModule::<T>::mint(&caller, unique_id, Color::Red).unwrap();

        // Set price for new collectible to avoid return Error(NotForSale)
        let new_price = T::Currency::free_balance(&caller);
		CollectiblesModule::<T>::set_price(RawOrigin::Signed(caller).into(), collectible_id,  Some(new_price)).unwrap();

        // Create new account and make free balance for it
		let receiver = account::<T::AccountId>("receiver", 1u32, 2u32);
        let bid_price = T::Currency::free_balance(&receiver);
	}: buy_collectible(RawOrigin::Signed(receiver.clone()), collectible_id,  bid_price)
	verify {
		assert_eq!(OwnerOfCollectibles::<T>::get(receiver), [unique_id].to_vec());
	}

	impl_benchmark_test_suite!(CollectiblesModule, crate::tests::new_test_ext(), crate::tests::Test);
}
