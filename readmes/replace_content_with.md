Replace the content with the content of another object that implements the `ToString` trait
---------------------------------------------------------
```rust
use string_builder::StringBuilder;
fn test_replace_with(){
    let mut sb = StringBuilder::new();
    sb.append("Hello, ");
    sb.replace_content_with("World!");
    println!("{}", sb.to_string()); // output will be "World!"
}
```