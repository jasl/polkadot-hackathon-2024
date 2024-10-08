// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_data_preservers
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("flashbox_dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_data_preservers
// --extrinsic
// *
// --chain=flashbox_dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/flashbox_weights/pallet_data_preservers.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_data_preservers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_data_preservers::WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::NextProfileId` (r:1 w:1)
	/// Proof: `DataPreservers::NextProfileId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn create_profile(_x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3593`
		// Minimum execution time: 57_751_000 picoseconds.
		Weight::from_parts(59_502_510, 3593)
			// Standard Error: 6_002
			.saturating_add(Weight::from_parts(68_177, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `DataPreservers::NextProfileId` (r:1 w:1)
	/// Proof: `DataPreservers::NextProfileId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn force_create_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3471`
		// Minimum execution time: 14_142_000 picoseconds.
		Weight::from_parts(14_457_926, 3471)
			// Standard Error: 90
			.saturating_add(Weight::from_parts(961, 0).saturating_mul(x.into()))
			// Standard Error: 1_889
			.saturating_add(Weight::from_parts(72_229, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn update_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3780`
		// Minimum execution time: 59_858_000 picoseconds.
		Weight::from_parts(60_969_027, 3780)
			// Standard Error: 189
			.saturating_add(Weight::from_parts(1_943, 0).saturating_mul(x.into()))
			// Standard Error: 3_952
			.saturating_add(Weight::from_parts(77_783, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn force_update_profile(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3780`
		// Minimum execution time: 52_271_000 picoseconds.
		Weight::from_parts(53_434_150, 3780)
			// Standard Error: 170
			.saturating_add(Weight::from_parts(980, 0).saturating_mul(x.into()))
			// Standard Error: 3_552
			.saturating_add(Weight::from_parts(72_319, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	fn delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3780`
		// Minimum execution time: 51_494_000 picoseconds.
		Weight::from_parts(52_766_000, 3780)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	fn force_delete_profile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3780`
		// Minimum execution time: 51_316_000 picoseconds.
		Weight::from_parts(52_098_000, 3780)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn start_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `373`
		//  Estimated: `3838`
		// Minimum execution time: 26_058_000 picoseconds.
		Weight::from_parts(26_707_000, 3838)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Registrar::ParaManager` (r:1 w:0)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn stop_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `3895`
		// Minimum execution time: 28_001_000 picoseconds.
		Weight::from_parts(28_671_000, 3895)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DataPreservers::Profiles` (r:1 w:1)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_start_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `3618`
		// Minimum execution time: 19_842_000 picoseconds.
		Weight::from_parts(20_308_000, 3618)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}