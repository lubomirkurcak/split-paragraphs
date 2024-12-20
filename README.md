# split-paragraphs

[![Build status](https://github.com/lubomirkurcak/split-paragraphs/workflows/tests/badge.svg)](https://github.com/lubomirkurcak/split-paragraphs/actions)
[![Build status](https://github.com/lubomirkurcak/split-paragraphs/workflows/no_std_build/badge.svg)](https://github.com/lubomirkurcak/split-paragraphs/actions)
[![Crates.io](https://img.shields.io/crates/v/split-paragraphs.svg)](https://crates.io/crates/split-paragraphs)

Provides an iterator over paragraphs of a string.

## Usage
```rust
use split_paragraphs::SplitParagraphs;

let text = "foo\r\nbar\n\nbaz\r";
let mut paragraphs = text.paragraphs();

assert_eq!(paragraphs.next(), Some("foo\r\nbar"));
assert_eq!(paragraphs.next(), Some("baz\r"));
assert_eq!(paragraphs.next(), None);
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
https://www.apache.org/licenses/LICENSE-2.0 or the MIT license
https://opensource.org/licenses/MIT, at your
option.
