#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_quantumfusion.
pub trait WeightInfo {
	fn verify_calldata() -> Weight;
	fn cause_error() -> Weight;
}

/// Weights for pallet_quantumfusion using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: QuantumFusionModule Calldata (r:0 w:1)
	/// Proof: QuantumFusionModule Calldata (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn verify_calldata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: QuantumFusionModule Calldata (r:1 w:1)
	/// Proof: QuantumFusionModule Calldata (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn cause_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: QuantumFusionModule Calldata (r:0 w:1)
	/// Proof: QuantumFusionModule Calldata (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn verify_calldata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: QuantumFusionModule Calldata (r:1 w:1)
	/// Proof: QuantumFusionModule Calldata (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn cause_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
