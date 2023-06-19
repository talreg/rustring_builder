Appending any object that implements the `ToString` trait
---------------------------------------------------------
```rust
use rustring_builder::StringBuilder;
fn try_appending(){
    let mut sb = StringBuilder::new();
    sb.append("Hello, ");
    sb.append(42);  // you can append numbers
    sb.to_string();
    println!("{}", sb.to_string()); // output will be "Hello, 42"
    sb.clear();
    sb.append(false);
    println!("{}", sb.to_string()); // output will be "false"
    
}
```
