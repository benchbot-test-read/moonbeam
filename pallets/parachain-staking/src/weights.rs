// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-17, STEPS: `1`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/moonbeam
// benchmark
// pallet
// --chain=dev
// --steps=1
// --repeat=1
// --pallet=parachain_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./benchmarking/frame-weight-template.hbs
// --output=./pallets/parachain-staking/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn set_staking_expectations() -> Weight;
	#[rustfmt::skip]
	fn set_inflation() -> Weight;
	#[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight;
	#[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight;
	#[rustfmt::skip]
	fn set_total_selected() -> Weight;
	#[rustfmt::skip]
	fn set_collator_commission() -> Weight;
	#[rustfmt::skip]
	fn set_blocks_per_round() -> Weight;
	#[rustfmt::skip]
	fn join_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn schedule_leave_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn execute_leave_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn cancel_leave_candidates(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn go_offline() -> Weight;
	#[rustfmt::skip]
	fn go_online() -> Weight;
	#[rustfmt::skip]
	fn candidate_bond_more() -> Weight;
	#[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight;
	#[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight;
	#[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight;
	#[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight;
	#[rustfmt::skip]
	fn execute_leave_delegators(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight;
	#[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight;
	#[rustfmt::skip]
	fn delegator_bond_more() -> Weight;
	#[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight;
	#[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight;
	#[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight;
	#[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight;
	#[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight;
	#[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn pay_one_collator_reward(y: u32, ) -> Weight;
	#[rustfmt::skip]
	fn base_on_initialize() -> Weight;
}

/// Weights for parachain_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_staking_expectations() -> Weight {
		(25_052_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_inflation() -> Weight {
		(58_869_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight {
		(21_433_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight {
		(21_757_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	#[rustfmt::skip]
	fn set_total_selected() -> Weight {
		(21_615_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	#[rustfmt::skip]
	fn set_collator_commission() -> Weight {
		(22_716_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_blocks_per_round() -> Weight {
		(65_441_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	#[rustfmt::skip]
	fn join_candidates(_x: u32, ) -> Weight {
		(146_820_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_candidates(_x: u32, ) -> Weight {
		(112_450_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_leave_candidates(_x: u32, ) -> Weight {
		(11_718_099_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(704 as Weight))
			.saturating_add(T::DbWeight::get().writes(704 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_candidates(_x: u32, ) -> Weight {
		(106_688_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_offline() -> Weight {
		(29_113_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_online() -> Weight {
		(28_297_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn candidate_bond_more() -> Weight {
		(48_113_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight {
		(28_175_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight {
		(64_931_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight {
		(23_203_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight {
		(83_422_000 as Weight)
			// Standard Error: 0
			.saturating_add((380_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 0
			.saturating_add((112_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight {
		(27_773_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	#[rustfmt::skip]
	fn execute_leave_delegators(_x: u32, ) -> Weight {
		(3_699_254_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(301 as Weight))
			.saturating_add(T::DbWeight::get().writes(202 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight {
		(24_303_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight {
		(33_472_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegator_bond_more() -> Weight {
		(64_793_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight {
		(33_995_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight {
		(81_973_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight {
		(73_884_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight {
		(29_432_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight {
		(40_119_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: System Account (r:302 w:301)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:9 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
	// Storage: ParachainStaking TopDelegations (r:9 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:10)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	// Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	#[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_636_000
			.saturating_add((44_022_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 5_000
			.saturating_add((257_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	#[rustfmt::skip]
	fn pay_one_collator_reward(_y: u32, ) -> Weight {
		(5_148_120_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(307 as Weight))
			.saturating_add(T::DbWeight::get().writes(303 as Weight))
	}
	#[rustfmt::skip]
	fn base_on_initialize() -> Weight {
		(4_860_000 as Weight)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_staking_expectations() -> Weight {
		(25_052_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_inflation() -> Weight {
		(58_869_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight {
		(21_433_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight {
		(21_757_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	#[rustfmt::skip]
	fn set_total_selected() -> Weight {
		(21_615_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	#[rustfmt::skip]
	fn set_collator_commission() -> Weight {
		(22_716_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
	fn set_blocks_per_round() -> Weight {
		(65_441_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	#[rustfmt::skip]
	fn join_candidates(_x: u32, ) -> Weight {
		(146_820_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_candidates(_x: u32, ) -> Weight {
		(112_450_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_leave_candidates(_x: u32, ) -> Weight {
		(11_718_099_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(704 as Weight))
			.saturating_add(RocksDbWeight::get().writes(704 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_candidates(_x: u32, ) -> Weight {
		(106_688_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_offline() -> Weight {
		(29_113_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn go_online() -> Weight {
		(28_297_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn candidate_bond_more() -> Weight {
		(48_113_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight {
		(28_175_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight {
		(64_931_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight {
		(23_203_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight {
		(83_422_000 as Weight)
			// Standard Error: 0
			.saturating_add((380_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 0
			.saturating_add((112_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight {
		(27_773_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	#[rustfmt::skip]
	fn execute_leave_delegators(_x: u32, ) -> Weight {
		(3_699_254_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(301 as Weight))
			.saturating_add(RocksDbWeight::get().writes(202 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight {
		(24_303_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight {
		(33_472_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn delegator_bond_more() -> Weight {
		(64_793_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight {
		(33_995_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight {
		(81_973_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight {
		(73_884_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight {
		(29_432_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight {
		(40_119_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: System Account (r:302 w:301)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:9 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
	// Storage: ParachainStaking TopDelegations (r:9 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:10)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	// Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	#[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_636_000
			.saturating_add((44_022_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 5_000
			.saturating_add((257_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(17 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	#[rustfmt::skip]
	fn pay_one_collator_reward(_y: u32, ) -> Weight {
		(5_148_120_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(307 as Weight))
			.saturating_add(RocksDbWeight::get().writes(303 as Weight))
	}
	#[rustfmt::skip]
	fn base_on_initialize() -> Weight {
		(4_860_000 as Weight)
	}
}
