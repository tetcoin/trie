
#![no_main]

use tetsy_trie_db_fuzz::fuzz_that_tetsy_reference_trie_root;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
	fuzz_that_tetsy_reference_trie_root(data);
});
