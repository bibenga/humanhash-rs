mod words;

use std::fmt;
use std::num::NonZeroUsize;

pub use uuid::Uuid;
pub use words::DEFAULT_WORD_LIST;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    TooFewInputBytes { input: usize, requested: usize },

    WordIndexOutOfRange { index: u8 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewInputBytes { input, requested } => write!(
                f,
                "fewer input bytes ({input}) than requested output words ({requested})"
            ),
            Self::WordIndexOutOfRange { index } => {
                write!(f, "word list has no entry for index {index}")
            }
        }
    }
}

impl std::error::Error for Error {}

const DEFAULT_WORDS: NonZeroUsize = match NonZeroUsize::new(4) {
    Some(n) => n,
    None => unreachable!(),
};

pub const DEFAULT_HASHER: HumanHasher<'static> =
    HumanHasher::new(DEFAULT_WORD_LIST, DEFAULT_WORDS, "-");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HumanHasher<'a> {
    wordlist: &'a [&'a str],
    words: NonZeroUsize,
    separator: &'a str,
}

impl<'a> HumanHasher<'a> {
    #[inline]
    #[must_use]
    pub const fn new(wordlist: &'a [&'a str], words: NonZeroUsize, separator: &'a str) -> Self {
        Self {
            wordlist,
            words,
            separator,
        }
    }

    #[inline]
    #[must_use]
    pub const fn wordlist(&self) -> &'a [&'a str] {
        self.wordlist
    }

    #[inline]
    #[must_use]
    pub const fn words(&self) -> NonZeroUsize {
        self.words
    }

    #[inline]
    #[must_use]
    pub const fn separator(&self) -> &'a str {
        self.separator
    }

    pub fn humanize(&self, data: &[u8]) -> Result<String, Error> {
        let words = self
            .compress(data)?
            .into_iter()
            .map(|index| {
                self.wordlist
                    .get(usize::from(index))
                    .copied()
                    .ok_or(Error::WordIndexOutOfRange { index })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(words.join(self.separator))
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let words = self.words.get();
        if words > data.len() {
            return Err(Error::TooFewInputBytes {
                input: data.len(),
                requested: words,
            });
        }

        let seg_size = data.len() / words;
        let mut checksums = vec![0u8; words];
        for (i, &byte) in data.iter().enumerate() {
            let segment = (i / seg_size).min(words - 1);
            checksums[segment] ^= byte;
        }
        Ok(checksums)
    }

    #[inline]
    pub fn humanize_uuid(&self, value: Uuid) -> Result<String, Error> {
        self.humanize(value.as_bytes())
    }

    #[inline]
    pub fn new_uuid4(&self) -> Result<(Uuid, String), Error> {
        self.with_humanized(Uuid::new_v4())
    }

    #[inline]
    pub fn new_uuid7(&self) -> Result<(Uuid, String), Error> {
        self.with_humanized(Uuid::now_v7())
    }

    #[inline]
    fn with_humanized(&self, value: Uuid) -> Result<(Uuid, String), Error> {
        let humanized = self.humanize_uuid(value)?;
        Ok((value, humanized))
    }
}

impl Default for HumanHasher<'static> {
    fn default() -> Self {
        DEFAULT_HASHER
    }
}

pub fn humanize(data: &[u8]) -> Result<String, Error> {
    DEFAULT_HASHER.humanize(data)
}

pub fn new_uuid4() -> Result<(Uuid, String), Error> {
    DEFAULT_HASHER.new_uuid4()
}

pub fn new_uuid7() -> Result<(Uuid, String), Error> {
    DEFAULT_HASHER.new_uuid7()
}
