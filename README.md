# Human hash

Original code was given from the repository - https://github.com/django-q2/django-q2/blob/master/django_q/humanhash.py

Turns arbitrary bytes (for example, a UUID) into a short phrase of words that is easy to read, compare and remember.

## Installation

```toml
[dependencies]
humanhash = "0.1"
```

## Usage

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let humanized = humanhash::humanize(&[96, 173, 141, 13, 135, 27, 96, 149, 128, 130, 151, 32])?;
    println!("Humanize: humanized={humanized}"); // equal-monkey-lake-double

    let (uuid, humanized) = humanhash::new_uuid4()?;
    println!("new_uuid4: uuid={uuid}; humanized={humanized}");

    let (uuid, humanized) = humanhash::new_uuid7()?;
    println!("new_uuid7: uuid={uuid}; humanized={humanized}");

    Ok(())
}
```

`Uuid` is re-exported from the [`uuid`](https://crates.io/crates/uuid) crate, so you don't need to depend on it directly.

### Custom hasher

The free functions use `DEFAULT_HASHER` (4 words, separated by `-`, built-in word list).
To change the number of words, the separator or the word list, create a `HumanHasher`:

```rust
use humanhash::{HumanHasher, DEFAULT_WORD_LIST};

let hasher = HumanHasher::new(DEFAULT_WORD_LIST, 2, "_");
assert_eq!(hasher.humanize(&[1, 2, 3, 4])?, "alaska_april");
```

The word list is borrowed, so it can also be loaded from a file, as long as it outlives the hasher.
It must contain 256 words (one per byte value), otherwise `humanize` returns `Error::WordIndexOutOfRange`.

### Errors

`humanize` returns `humanhash::Error`:

- `ZeroWords` - the hasher was configured with zero words;
- `TooFewInputBytes` - the input has fewer bytes than the requested number of words;
- `WordIndexOutOfRange` - the word list is shorter than 256 words.

## Command line

The command is built behind the `cli` feature, so the library itself doesn't depend on `clap`:

```sh
cargo install --path . --features cli

humanize --value=938AEF34-912E-4729-8C30-7DC347504688
humanize --value=123456 --format=text
humanize   # a new random UUID
```

## Tests

```sh
cargo test
cargo test --features cli
```
