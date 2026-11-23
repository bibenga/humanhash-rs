use humanhash::{DEFAULT_HASHER, DEFAULT_WORD_LIST, humanize, new_uuid4, new_uuid7};

fn expected_phrase(bytes: &[u8]) -> String {
    bytes
        .chunks(4)
        .map(|chunk| {
            let index = chunk.iter().fold(0u8, |acc, &byte| acc ^ byte);
            DEFAULT_WORD_LIST[usize::from(index)]
        })
        .collect::<Vec<_>>()
        .join("-")
}

#[test]
fn humanize_short_input() {
    let value = humanize(&[1, 2, 3, 4, 5]).expect("humanize failed");
    assert_eq!(value, "alabama-alanine-alaska-alabama");
}

#[test]
fn humanize_twelve_bytes() {
    let data = [96, 173, 141, 13, 135, 27, 96, 149, 128, 130, 151, 32];
    let value = humanize(&data).expect("humanize failed");
    assert_eq!(value, "equal-monkey-lake-double");
}

#[test]
fn humanize_uuid_matches_humanize_bytes() {
    let (value, humanized) = new_uuid4().expect("new_uuid4 failed");
    assert_eq!(
        DEFAULT_HASHER.humanize_uuid(value).as_deref(),
        Ok(humanized.as_str())
    );
}

#[test]
fn new_uuid4_matches_manual_humanization() {
    let (value, humanized) = new_uuid4().expect("new_uuid4 failed");
    assert_eq!(humanized, expected_phrase(value.as_bytes()));
}

#[test]
fn new_uuid7_matches_manual_humanization() {
    let (value, humanized) = new_uuid7().expect("new_uuid7 failed");
    assert_eq!(humanized, expected_phrase(value.as_bytes()));
}
