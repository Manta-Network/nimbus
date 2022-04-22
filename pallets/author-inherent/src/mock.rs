// Copyright 2019-2021 PureStake Inc.
// This file is part of Nimbus.

// Nimbus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Nimbus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Nimbus.  If not, see <http://www.gnu.org/licenses/>.

use crate as pallet_testing;
use frame_support::parameter_types;
use frame_support::sp_io;
use frame_support::traits::ConstU32;
use frame_support::weights::{RuntimeDbWeight, Weight};
use frame_system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use cumulus_pallet_parachain_system::ParachainSetCode;

use cumulus_primitives_core::{ParaId};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		AuthorInherent: pallet_testing::{Pallet, Call, Storage},
		ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Storage, Inherent, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub Authors: Vec<u64> = vec![1, 2, 3, 4, 5];
	pub const TestDbWeight: RuntimeDbWeight = RuntimeDbWeight {
		read: 1,
		write: 10,
	};
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = TestDbWeight;
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ParachainSetCode<Self>;
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const ParachainId: ParaId = ParaId::new(200);
	pub const ReservedXcmpWeight: Weight = 0;
	pub const ReservedDmpWeight: Weight = 0;
}

impl cumulus_pallet_parachain_system::Config for Test {
	type Event = Event;
	type OnSystemEvent = ();
	type SelfParaId = ParachainId;
	type DmpMessageHandler = ();
	type ReservedDmpWeight = ReservedDmpWeight;
	type OutboundXcmpMessageSource = ();
	type XcmpMessageHandler = ();
	type ReservedXcmpWeight = ReservedXcmpWeight;
}

impl pallet_testing::Config for Test {
	type SlotBeacon = cumulus_pallet_parachain_system::RelaychainBlockNumberProvider<Self>;
	type AccountLookup = ();
	type EventHandler = ();
	type CanAuthor = ();
}

/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
