// Copyright 2017, 2020 Parity Technologies
//
// Licensed under the Apache License, Version .0 (the "License");
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

use trie_db::DBValue;
use tetsy_memory_db::{MemoryDB, HashKey, PrefixedKey};
use tetsy_keccak_hasher::KeccakHasher;

#[test]
fn trie_root_empty () {
	compare_implementations(vec![])
}

#[test]
fn trie_one_node () {
	compare_implementations(vec![
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8]),
	]);
}

#[test]
fn root_extension_one () {
	compare_implementations(vec![
		(vec![1u8, 2u8, 3u8, 3u8], vec![8u8;32]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;32]),
	]);
}

fn test_iter(data: Vec<(Vec<u8>, Vec<u8>)>) {
	use tetsy_reference_trie::{RefTrieDBMut, RefTrieDB};
	use trie_db::{TrieMut, Trie};

	let mut db = MemoryDB::<KeccakHasher, PrefixedKey<_>, DBValue>::default();
	let mut root = Default::default();
	{
		let mut t = RefTrieDBMut::new(&mut db, &mut root);
		for i in 0..data.len() {
			let key: &[u8]= &data[i].0;
			let value: &[u8] = &data[i].1;
			t.insert(key, value).unwrap();
		}
	}
	let t = RefTrieDB::new(&db, &root).unwrap();
	for (i, kv) in t.iter().unwrap().enumerate() {
		let (k, v) = kv.unwrap();
		let key: &[u8]= &data[i].0;
		let value: &[u8] = &data[i].1;
		assert_eq!(k, key);
		assert_eq!(v, value);
	}
	for (k, v) in data.into_iter() {
		assert_eq!(&t.get(&k[..]).unwrap().unwrap()[..], &v[..]);
	}
}

fn test_iter_no_extension(data: Vec<(Vec<u8>, Vec<u8>)>) {
	use tetsy_reference_trie::{RefTrieDBMutNoExt, RefTrieDBNoExt};
	use trie_db::{TrieMut, Trie};

	let mut db = MemoryDB::<KeccakHasher, PrefixedKey<_>, DBValue>::default();
	let mut root = Default::default();
	{
		let mut t = RefTrieDBMutNoExt::new(&mut db, &mut root);
		for i in 0..data.len() {
			let key: &[u8]= &data[i].0;
			let value: &[u8] = &data[i].1;
			t.insert(key, value).unwrap();
		}
	}
	let t = RefTrieDBNoExt::new(&db, &root).unwrap();
	for (i, kv) in t.iter().unwrap().enumerate() {
		let (k, v) = kv.unwrap();
		let key: &[u8]= &data[i].0;
		let value: &[u8] = &data[i].1;
		assert_eq!(k, key);
		assert_eq!(v, value);
	}
	for (k, v) in data.into_iter() {
		assert_eq!(&t.get(&k[..]).unwrap().unwrap()[..], &v[..]);
	}
}

fn compare_implementations(data: Vec<(Vec<u8>, Vec<u8>)>) {
	test_iter(data.clone());
	test_iter_no_extension(data.clone());
	compare_implementations_h(data.clone());
	compare_implementations_prefixed(data.clone());
	compare_implementations_no_extension(data.clone());
	compare_implementations_no_extension_prefixed(data.clone());
}

fn compare_implementations_prefixed(data: Vec<(Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, PrefixedKey<_>, _>::default();
	let hashdb = MemoryDB::<KeccakHasher, PrefixedKey<_>, DBValue>::default();
	tetsy_reference_trie::compare_implementations(data, memdb, hashdb);
}
fn compare_implementations_h(data: Vec<(Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, HashKey<_>, _>::default();
	let hashdb = MemoryDB::<KeccakHasher, HashKey<_>, DBValue>::default();
	tetsy_reference_trie::compare_implementations(data, memdb, hashdb);
}
fn compare_implementations_no_extension(data: Vec<(Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, HashKey<_>, _>::default();
	let hashdb = MemoryDB::<KeccakHasher, HashKey<_>, DBValue>::default();
	tetsy_reference_trie::compare_implementations_no_extension(data, memdb, hashdb);
}
fn compare_implementations_no_extension_prefixed(data: Vec<(Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, PrefixedKey<_>, _>::default();
	let hashdb = MemoryDB::<KeccakHasher, PrefixedKey<_>, DBValue>::default();
	tetsy_reference_trie::compare_implementations_no_extension(data, memdb, hashdb);
}
fn compare_implementations_no_extension_unordered(data: Vec<(Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, HashKey<_>, _>::default();
	let hashdb = MemoryDB::<KeccakHasher, HashKey<_>, DBValue>::default();
	tetsy_reference_trie::compare_implementations_no_extension_unordered(data, memdb, hashdb);
}
fn compare_no_extension_insert_remove(data: Vec<(bool, Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, PrefixedKey<_>, _>::default();
	tetsy_reference_trie::compare_no_extension_insert_remove(data, memdb);
}
fn compare_root(data: Vec<(Vec<u8>, Vec<u8>)>) {
	let memdb = MemoryDB::<_, HashKey<_>, _>::default();
	tetsy_reference_trie::compare_root(data, memdb);
}
fn compare_unhashed(data: Vec<(Vec<u8>, Vec<u8>)>) {
	tetsy_reference_trie::compare_unhashed(data);
}
fn compare_unhashed_no_extension(data: Vec<(Vec<u8>, Vec<u8>)>) {
	tetsy_reference_trie::compare_unhashed_no_extension(data);
}

// Following tests are a bunch of detected issue here for non regression.

#[test]
fn trie_middle_node1 () {
	compare_implementations(vec![
		(vec![1u8, 2u8], vec![8u8;32]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;32]),
	]);
}
#[test]
fn trie_middle_node2 () {
	compare_implementations(vec![
		(vec![0u8, 2u8, 3u8, 5u8, 3u8], vec![1u8;32]),
		(vec![1u8, 2u8], vec![8u8;32]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;32]),
		(vec![1u8, 2u8, 3u8, 5u8], vec![7u8;32]),
		(vec![1u8, 2u8, 3u8, 5u8, 3u8], vec![7u8;32]),
	]);
}
#[test]
fn root_extension_bis () {
	compare_root(vec![
		(vec![1u8, 2u8, 3u8, 3u8], vec![8u8;32]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;32]),
	]);
}
#[test]
fn root_extension_tierce () {
	let d = vec![
		(vec![1u8, 2u8, 3u8, 3u8], vec![8u8;2]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;2]),
	];
	compare_unhashed(d.clone());
	compare_unhashed_no_extension(d);
}
#[test]
fn root_extension_tierce_big () {
	// on more content unhashed would hash
	compare_unhashed(vec![
		(vec![1u8, 2u8, 3u8, 3u8], vec![8u8;32]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;32]),
		(vec![1u8, 6u8, 3u8, 3u8], vec![8u8;32]),
		(vec![6u8, 2u8, 3u8, 3u8], vec![8u8;32]),
		(vec![6u8, 2u8, 3u8, 13u8], vec![8u8;32]),
	]);
}
#[test]
fn trie_middle_node2x () {
	compare_implementations(vec![
		(vec![0u8, 2u8, 3u8, 5u8, 3u8], vec![1u8;2]),
		(vec![1u8, 2u8], vec![8u8;2]),
		(vec![1u8, 2u8, 3u8, 4u8], vec![7u8;2]),
		(vec![1u8, 2u8, 3u8, 5u8], vec![7u8;2]),
		(vec![1u8, 2u8, 3u8, 5u8, 3u8], vec![7u8;2]),
	]);
}
#[test]
fn fuzz1 () {
	compare_implementations(vec![
		(vec![01u8], vec![42u8, 9]),
		(vec![01u8, 0u8], vec![0u8, 0]),
		(vec![255u8, 2u8], vec![1u8, 0]),
	]);
}
#[test]
fn fuzz2 () {
	compare_implementations(vec![
		(vec![0, 01u8], vec![42u8, 9]),
		(vec![0, 01u8, 0u8], vec![0u8, 0]),
		(vec![0, 255u8, 2u8], vec![1u8, 0]),
	]);
}
#[test]
fn fuzz3 () {
	compare_implementations(vec![
		(vec![0], vec![196, 255]),
		(vec![48], vec![138, 255]),
		(vec![67], vec![0, 0]),
		(vec![128], vec![255, 0]),
		(vec![247], vec![0, 196]),
		(vec![255], vec![0, 0]),
	]);
}
#[test]
fn fuzz_no_extension1 () {
	compare_implementations(vec![
		(vec![0], vec![128, 0]),
		(vec![128], vec![0, 0]),
	]);
}
#[test]
fn fuzz_no_extension2 () {
	compare_implementations(vec![
		(vec![0], vec![6, 255]),
		(vec![6], vec![255, 186]),
		(vec![255], vec![186, 255]),
	]);
}
#[test]
fn fuzz_no_extension5 () {
	compare_implementations(vec![
		(vec![0xaa], vec![0xa0]),
		(vec![0xaa, 0xaa], vec![0xaa]),
		(vec![0xaa, 0xbb], vec![0xab]),
		(vec![0xbb], vec![0xb0]),
		(vec![0xbb, 0xbb], vec![0xbb]),
		(vec![0xbb, 0xcc], vec![0xbc]),
	]);
}
#[test]
fn fuzz_no_extension3 () {
	compare_implementations(vec![
		(vec![0], vec![0, 0]),
		(vec![11, 0], vec![0, 0]),
		(vec![11, 252], vec![11, 0]),
	]);

	compare_implementations_no_extension_unordered(vec![
		(vec![11, 252], vec![11, 0]),
		(vec![11, 0], vec![0, 0]),
		(vec![0], vec![0, 0]),
	]);
}
#[test]
fn fuzz_no_extension4 () {
	compare_implementations_no_extension(vec![
		(vec![0x01, 0x56], vec![0x1]),
		(vec![0x02, 0x42], vec![0x2]),
		(vec![0x02, 0x50], vec![0x3]),
	]);
}
#[test]
fn fuzz_no_extension_insert_remove_1 () {
	let data = vec![
		(false, vec![0], vec![251, 255]),
		(false, vec![0, 1], vec![251, 255]),
		(false, vec![0, 1, 2], vec![255; 32]),
		(true, vec![0, 1], vec![0, 251]),
	];
	compare_no_extension_insert_remove(data);
}
#[test]
fn fuzz_no_extension_insert_remove_2 () {
	let data = vec![
		(false, vec![0x00], vec![0xfd, 0xff]),
		(false, vec![0x10, 0x00], vec![1;32]),
		(false, vec![0x11, 0x10], vec![0;32]),
		(true, vec![0x10, 0x00], vec![])
	];
	compare_no_extension_insert_remove(data);
}
#[test]
fn two_bytes_nibble_length () {
	let data = vec![
		(vec![00u8], vec![0]),
		(vec![01u8;64], vec![0;32]),
	];
	compare_implementations_no_extension(data.clone());
	compare_implementations_no_extension_prefixed(data.clone());
}
#[test]
#[should_panic]
fn too_big_nibble_length_old () {
	compare_implementations_h(vec![
		(vec![01u8;64], vec![0;32]),
	]);
}
#[test]
fn too_big_nibble_length_new () {
	compare_implementations_no_extension(vec![
		(vec![01u8;((u16::max_value() as usize + 1) / 2) + 1], vec![0;32]),
	]);
}
#[test]
fn polka_re_test () {
	compare_implementations(vec![
		(vec![77, 111, 111, 55, 111, 104, 121, 97], vec![68, 97, 105, 55, 105, 101, 116, 111]),
		(vec![101, 105, 67, 104, 111, 111, 66, 56], vec![97, 56, 97, 113, 117, 53, 97]),
		(vec![105, 97, 48, 77, 101, 105, 121, 101], vec![69, 109, 111, 111, 82, 49, 97, 105]),
	]);
}
