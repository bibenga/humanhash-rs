mod words;

#[cfg(test)]
mod tests;

use std::fmt;
pub use uuid::Uuid;
pub use words::DEFAULT_WORD_LIST;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    WordListInvalidSize { size: usize },
    ZeroWords,
    TooFewInputBytes { input: usize, requested: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WordListInvalidSize { size } => {
                write!(f, "word list must contain exactly 256 words, got {size}")
            }
            Self::ZeroWords => write!(f, "number of output words must be greater than zero"),
            Self::TooFewInputBytes { input, requested } => write!(
                f,
                "fewer input bytes ({input}) than requested output words ({requested})"
            ),
        }
    }
}

impl std::error::Error for Error {}

pub const DEFAULT_WORDS: usize = 4;

pub const DEFAULT_HASHER: HumanHasher<'static> =
    match HumanHasher::new(DEFAULT_WORD_LIST, DEFAULT_WORDS, "-") {
        Ok(h) => h,
        Err(_) => unreachable!(),
    };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HumanHasher<'a> {
    wordlist: &'a [&'a str],
    words: usize,
    separator: &'a str,
}

impl<'a> HumanHasher<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        wordlist: &'a [&'a str],
        words: usize,
        separator: &'a str,
    ) -> Result<Self, Error> {
        if wordlist.len() != 256 {
            return Err(Error::WordListInvalidSize {
                size: wordlist.len(),
            });
        }
        if words == 0 {
            return Err(Error::ZeroWords);
        }
        Ok(Self {
            wordlist,
            words,
            separator,
        })
    }

    #[inline]
    #[must_use]
    pub const fn wordlist(&self) -> &'a [&'a str] {
        self.wordlist
    }

    #[inline]
    #[must_use]
    pub const fn words(&self) -> usize {
        self.words
    }

    #[inline]
    #[must_use]
    pub const fn separator(&self) -> &'a str {
        self.separator
    }

    pub fn humanize(&self, data: &[u8]) -> Result<String, Error> {
        let words: Vec<&str> = self
            .compress(data)?
            .into_iter()
            .map(|index| self.wordlist[usize::from(index)])
            .collect();
        Ok(words.join(self.separator))
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let words = self.words;

        if words > data.len() {
            return Err(Error::TooFewInputBytes {
                input: data.len(),
                requested: words,
            });
        }

        let seg_size = data.len() / words;
        let mut checksums = vec![0u8; words];
        for (i, &byte) in data.iter().enumerate() {
            let seg = (i / seg_size).min(words - 1);
            checksums[seg] ^= byte;
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
