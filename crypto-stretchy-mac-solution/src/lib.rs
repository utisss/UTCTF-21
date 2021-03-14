#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_debug_implementations)]

const SHA224_DIGEST_LENGTH: usize = 224 / 8;

type ShaLong = u32;

pub fn sha224_extended_hash(
    initial_state: &[u8; SHA224_DIGEST_LENGTH], exposed_state: u32,
    prior_blocks: usize, m: &[u8],
) -> [u8; SHA224_DIGEST_LENGTH] {
    let mut md = [0; SHA224_DIGEST_LENGTH];

    unsafe {
        c::sha224_extended_hash(
            initial_state.as_ptr(),
            exposed_state,
            prior_blocks,
            m.as_ptr(),
            m.len(),
            md.as_mut_ptr(),
        );
    }

    md
}

pub fn sha224_length_extension_attack(
    possible_hashes: &mut Vec<[u8; SHA224_DIGEST_LENGTH]>,
    initial_state: &[u8; SHA224_DIGEST_LENGTH], prior_blocks: usize, m: &[u8],
) -> Option<([u8; SHA224_DIGEST_LENGTH], u32)> {
    possible_hashes.sort_unstable();

    let result = unsafe {
        c::sha224_length_extension_attack(
            possible_hashes.as_ptr(),
            possible_hashes.len(),
            initial_state.as_ptr(),
            prior_blocks,
            m.as_ptr(),
            m.len(),
        )
    };

    possible_hashes
        .get((result >> 32) as usize)
        .map(|&digest| (digest, (result & 0xFFFFFFFF) as u32))
}

mod c {
    use super::{
        ShaLong,
        SHA224_DIGEST_LENGTH,
    };
    use std::os::raw::c_uchar;

    extern {
        pub(super) fn sha224_extended_hash(
            initial_state: *const c_uchar, exposed_state: ShaLong,
            prior_blocks: usize, m: *const c_uchar, m_len: usize,
            md: *mut c_uchar,
        ) -> *const c_uchar;

        pub(super) fn sha224_length_extension_attack(
            possible_hashes: *const [c_uchar; SHA224_DIGEST_LENGTH],
            possible_hashes_length: usize, initial_state: *const c_uchar,
            prior_blocks: usize, m: *const c_uchar, m_len: usize,
        ) -> u64;
    }
}
