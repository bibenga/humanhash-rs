use std::num::NonZeroUsize;

use crate::{DEFAULT_HASHER, DEFAULT_WORD_LIST, Error, HumanHasher};

#[test]
fn compress_same_length_is_identity() {
    assert_eq!(DEFAULT_HASHER.compress(&[1, 2, 3, 4]), Ok(vec![1, 2, 3, 4]));
}

#[test]
fn compress_remainder_goes_to_last_segment() {
    assert_eq!(
        DEFAULT_HASHER.compress(&[1, 2, 3, 4, 5]),
        Ok(vec![1, 2, 3, 4 ^ 5])
    );
}

#[test]
fn compress_xors_each_segment() {
    let data: [u8; 12] = [
        0x60, 0xAD, 0x8D, 0x0D, 0x87, 0x1B, 0x60, 0x95, 0x80, 0x82, 0x97, 0x20,
    ];
    let expected = vec![
        0x60 ^ 0xAD ^ 0x8D,
        0x0D ^ 0x87 ^ 0x1B,
        0x60 ^ 0x95 ^ 0x80,
        0x82 ^ 0x97 ^ 0x20,
    ];
    assert_eq!(DEFAULT_HASHER.compress(&data), Ok(expected));
}

#[test]
fn compress_rejects_input_shorter_than_words() {
    assert_eq!(
        DEFAULT_HASHER.compress(&[1, 2, 3]),
        Err(Error::TooFewInputBytes {
            input: 3,
            requested: 4
        })
    );
}

#[test]
fn humanize_reports_index_missing_from_word_list() {
    let words = NonZeroUsize::new(1).unwrap();
    let hasher = HumanHasher::new(&["only"], words, "-");
    assert_eq!(
        hasher.humanize(&[7]),
        Err(Error::WordIndexOutOfRange { index: 7 })
    );
}

#[test]
fn humanize_uses_custom_separator() {
    let words = NonZeroUsize::new(2).unwrap();
    let hasher = HumanHasher::new(DEFAULT_WORD_LIST, words, "_");
    assert_eq!(hasher.humanize(&[1, 2]), Ok("alabama_alanine".to_owned()));
}
