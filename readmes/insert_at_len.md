insert_at_with_len
==============
Insert a string at a given position to the buffer.
```rust
use rustring_builder::StringBuilder;
fn test_insert_at_with_len(){
    let mut builder = StringBuilder::new();
    builder.append("hello world!");
    let sentence=" cruel, isn't it?";
    builder.insert_at_with_len(5, sentence, 6);
    assert_eq!("hello cruel world!", builder.to_string());
}
```