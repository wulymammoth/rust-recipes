# rust recipes

## strings

- forces the developer to handle the complexity of strings
- simple things are more challenging

- strings
  - stored on the heap
  - mutable
- string slices
  - immutable (with some exceptions)
  - stack-allocated and sometimes as a heap reference, and sometime embedded within the code

- easily translatable between the two types

* characters are not a single character string
  - contains much more info

* good way of getting a string slice from a string is using the `&` borrow operator (reference)

---

- byte is the smallest possible way to encode a single character
- strings can be 8-bit (1 byte), 16-bit (2 bytes), 24-bit (3 bytes), and 32-bits (4 bytes)
* `u8` in Rust just represents the BYTES of memory encoding the string and NOT the individual characters
* common mistake: take a substring based on a byte index that splits a character results in a panic
* `chars()` for Unicode codepoints and `[]` for bytes
  - mixing these up will result in panics/crashes

- strings are stored in `Vec<u8>` but when the machine reads them it looks at the first byte and checcks if the most significant bit is `0` or `1`. If it finds the leading bit to be `0` it knows the that the UTF-8 character is only 1 byte(u8) long and reads it. If it finds a `1` instead, it then realizes that the UTF-8 character is at least 2 bytes and needs to look at the next byte
  - it'll read the first 128 ASCII characters fine, but gibberish beyond that
