// Copyright 2017, 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use tetsy_memory_db::{MemoryDB, HashKey};
use tetsy_trie_db::{DBValue, Trie, TrieMut};
use tetsy_keccak_hasher::KeccakHasher;
use tetsy_reference_trie::{RefFatDBMut, RefFatDB};

#[test]
fn fatdb_to_trie() {
	let mut memdb = MemoryDB::<KeccakHasher, HashKey<_>, DBValue>::default();
	let mut root = Default::default();
	{
		let mut t = RefFatDBMut::new(&mut memdb, &mut root);
		t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
	}
	let t = RefFatDB::new(&memdb, &root).unwrap();
	assert_eq!(t.get(&[0x01u8, 0x23]).unwrap().unwrap(), vec![0x01u8, 0x23]);
	assert_eq!(
		t.iter().unwrap().map(Result::unwrap).collect::<Vec<_>>(),
		vec![(vec![0x01u8, 0x23], vec![0x01u8, 0x23])]
	);
}
