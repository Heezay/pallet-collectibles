//! The crate's tests.

#[cfg(test)]
mod tests;

use crate as pallet_collectibles;
use frame_support::{
	assert_noop, assert_ok,
	traits::{ConstU16, ConstU32, ConstU64, Currency},
};
use frame_system as system;

use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

pub const U64_MAX: u64 = u64::MAX;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;
// TODO: Resolve problem with Err(InsufficientBalance)
// type DepositBalanceOf<T> = <<T as pallet_collectibles::Config>::Currency as Currency<
// 	<T as frame_system::Config>::AccountId,
// >>::Balance;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		CollectiblesModule: pallet_collectibles,
		Balances: pallet_balances,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_collectibles::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CollectionRandomness = RandomnessCollectiveFlip;
	type MaximumOwned = frame_support::pallet_prelude::ConstU32<100>;
}

impl pallet_balances::Config for Test {
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type MaxLocks = ConstU32<10>;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

// Test environment function to call origin
fn origin_for(account_id: u64) -> system::RawOrigin<u64> {
	system::Origin::<Test>::Signed(account_id)
}

// Test environment function to create collectible and return it
fn create_collectible() -> ([u8; 16], u64) {
	let (unique_id, color) = CollectiblesModule::gen_unique_id();
	let minter: u64 = 1;
	CollectiblesModule::mint(&minter, unique_id, color).unwrap();
	(unique_id, minter)
}
