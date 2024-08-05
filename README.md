# split-paragraphs

Provides an iterator over the paragraphs of a string.

## Usage
```rust
use split_paragraphs::ParagraphsExt;

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
option. This file may not be copied, modified, or distributed
except according to those terms.
