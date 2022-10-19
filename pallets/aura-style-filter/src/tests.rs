// Copyright 2019-2022 PureStake Inc.
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

use crate::mock::*;
use nimbus_primitives::CanAuthor;

#[test]
fn sanity_check_aura_sequence() {
	#[rustfmt::skip]
	let expected_sequence: Vec<u64> = vec![
		1,1,2,2,3,3,4,4,5,5,1,1,2,2
	];
	let mock_authors = <Test as crate::Config>::PotentialAuthors::get().clone();
	let mut sequence = vec![];
	for slot in 0..expected_sequence.len() as u32 {
		let eligibles: Vec<_> = mock_authors
			.iter()
			.filter(|&account| AuraStyleFilter::can_author(account, &slot))
			.collect();
		match eligibles.len() {
			0 => panic!(), // make sure *someone* is always eligible
			1 => sequence.push(*eligibles[0]),
			_ => panic!(), // make sure we don't get multiple eligible authors per slot
		}
	}
	assert_eq!(expected_sequence, sequence);
}
