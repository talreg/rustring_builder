## String Builder - reusable char buffer
Basic usage:
```rust
use rustring_builder::StringBuilder;
fn add_strings_example()
{
    let mut sb = StringBuilder::new();
    sb.append("Hello");
    sb.append(" ");
    sb.append("World");
    sb.append("!");
    let s = sb.to_string();
    assert_eq!(s, "Hello World!");
}
```
* The StringBuilder object allows you to add any object (including custom types)
as long as they implement the ToString trait:
```rust
use rustring_builder::StringBuilder;
fn add_types_example()
{
    let mut sb = StringBuilder::new();
    sb.append(1);
    sb.append(" ");
    sb.append(2.0);
    sb.append(" ");
    sb.append("three");
    sb.append(" ");
    sb.append(true);
    let s = sb.to_string();
    assert_eq!(s, "1 2 three true");
}
```
* StringBuilder doesn't consume the object appended
* StringBuilder is not consumed by calling the to_string() method.
```rust
use rustring_builder::StringBuilder;
fn append_string_builder_example()
{
    let mut sb = StringBuilder::new();
    sb.append("Hello");
    sb.append(" ");
    sb.append("World");
    sb.append("!");
    let s = sb.to_string();
    assert_eq!(s, "Hello World!");
    sb.append(" Goodbye!");
    let s = sb.to_string();
    assert_eq!(s, "Hello World! Goodbye!");
}
```
* StringBuilder can be appended to another StringBuilder, using simple += operator
```rust
use rustring_builder::StringBuilder;
fn add_another_string_builder_example()
{
    let mut sb = StringBuilder::new();
    sb.append("Hello");
    sb.append(" ");
    sb.append("World");
    sb.append("!");
    let s = sb.to_string();
    assert_eq!(s, "Hello World!");
    let mut sb2 = StringBuilder::new();
    sb2.append(" Goodbye!");
    sb += sb2;
    let s = sb.to_string();
    assert_eq!(s, "Hello World! Goodbye!");
}
```
in this example, sb2 is consumed by the += operator, and can no longer be used.

* There is a push method that allows you to push a character onto the string builder
```rust
use rustring_builder::StringBuilder;
fn push_example()
{
    let mut sb = StringBuilder::new();
    sb.append("Hello");
    sb.append(" ");
    sb.append("World");
    sb.append("!");
    let s = sb.to_string();
    assert_eq!(s, "Hello World!");
    sb.push(' ');
    sb.push('G');
    sb.push('o');
    sb.push('o');
    sb.push('d');
    sb.push('b');
    sb.push('y');
    sb.push('e');
    sb.push('!');
    let s = sb.to_string();
    assert_eq!(s, "Hello World! Goodbye!");
}
```
This method might be useful if you are building a string from a stream of characters, or if you are building a string from a file.
* You can access characters in the string builder using the nth method
```rust
use rustring_builder::StringBuilder;
fn nth_example()
{
    let mut sb = StringBuilder::new();
    sb.append("Hello");
    sb.append(" ");
    sb.append("World");
    sb.append("!");
    let s = sb.to_string();
    assert_eq!(s, "Hello World!");
    assert_eq!(sb.nth(0), Some('H'));
    assert_eq!(sb.nth(1), Some('e'));
    assert_eq!(sb.nth(2), Some('l'));
    assert_eq!(sb.nth(3), Some('l'));
    assert_eq!(sb.nth(4), Some('o'));
    assert_eq!(sb.nth(5), Some(' '));
    assert_eq!(sb.nth(6), Some('W'));
    assert_eq!(sb.nth(7), Some('o'));
    assert_eq!(sb.nth(8), Some('r'));
    assert_eq!(sb.nth(9), Some('l'));
    assert_eq!(sb.nth(10), Some('d'));
    assert_eq!(sb.nth(11), Some('!'));
    assert_eq!(sb.nth(12), None);
}
```

## Consuming methods:
* Operators, such as + or += will consume the respective value.
* Iterator will also consume the value
* Direct access (nth) will **not** consume the value.