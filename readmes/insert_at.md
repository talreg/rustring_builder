insert_at
==============
Insert a string at a given position to the buffer.
```rust
use rustring_builder::StringBuilder;
fn test_insert_at(){
    let mut builder = StringBuilder::new();
    builder.append("hello world!");
    builder.insert_at(5, " cruel");
    assert_eq!("hello cruel world!", builder.to_string());

}
```