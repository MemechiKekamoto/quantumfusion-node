//! Benchmarking setup for pallet-offchaingate
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as OffchainGate;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn verify_calldata() {
		let value = 100u32.into();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		verify_calldata(RawOrigin::Signed(caller), value);

		assert_eq!(Calldata::<T>::get(), Some(value));
	}

	#[benchmark]
	fn cause_error() {
		Calldata::<T>::put(100u32);
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));

		assert_eq!(Calldata::<T>::get(), Some(101u32));
	}

	impl_benchmark_test_suite!(OffchainGate, crate::mock::new_test_ext(), crate::mock::Test);
}
