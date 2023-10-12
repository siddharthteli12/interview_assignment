//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Dao;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn init_setup<T: Config>() -> T::AccountId {
	let caller: T::AccountId = whitelisted_caller();
	<T as Config>::Currency::issue(
		<T as Config>::Currency::minimum_balance() * 1_000_000u32.into(),
	);
	<T as Config>::Currency::make_free_balance_be(
		&caller,
		<T as Config>::Currency::minimum_balance() * 1_000_000u32.into(),
	);
	caller
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_dao() {
		let owner = init_setup::<T>();
		#[extrinsic_call]
		create_dao(
			RawOrigin::Signed(owner.clone()),
			b"TestName".to_vec(),
			b"Info".to_vec(),
			b"Hash".to_vec(),
		);
	}
	#[benchmark]
	fn delete_dao() {
		let owner = init_setup::<T>();
		Dao::<T>::create_dao(
			RawOrigin::Signed(owner.clone()).into(),
			b"TestName".to_vec(),
			b"Info".to_vec(),
			b"Hash".to_vec(),
		);
		#[extrinsic_call]
		delete_dao(RawOrigin::Signed(owner.clone()), 1u128)
	}

	#[benchmark]
	fn transfer_dao_ownership() {
		let owner = init_setup::<T>();
		let new_owner: T::AccountId = account("new owner", 0, 0);
		Dao::<T>::create_dao(
			RawOrigin::Signed(owner.clone()).into(),
			b"TestName".to_vec(),
			b"Info".to_vec(),
			b"Hash".to_vec(),
		);
		#[extrinsic_call]
		transfer_dao_ownership(RawOrigin::Signed(owner.clone()), 1u128, new_owner)
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
