# Vigenère Cipher
[![Current Version](http://meritbadge.herokuapp.com/vigenere-cipher)](https://crates.io/crates/vigenere-cipher)

An implementation of the Vigènere cipher in Rust. This particular cipher is historical and doesn't serve the purpose to be used in modern applications that require true application. 

## Getting Started
### Prerequistes
First, [Nightly Rust](https://doc.rust-lang.org/1.13.0/book/nightly-rust.html) needs to be installed
### Installing
First, add the following to `Cargo.toml`:

```toml
[dependencies]
vigenere-cipher = "0.1.0"
```

Then, add this to any Rust file:
``` Rust
extern crate vigenere_cipher;

use vigenere_cipher::*;
```

## Running the tests
Enter the vigenere-cipher repository and run `cargo +nightly test`

## Authors
* **Lukas A. Mueller** - *Initial work*

# Versioning

[SemVer](http://semver.org/) applies for versioning.

## License
This project is licensed under GNU GPLv3 - see the [LICENSE file](/LICENSE) for details

## Acknowledgments
* [gitignore.io](https://www.gitignore.io)
* [choosealicense.com](https://choosealicense.com)
