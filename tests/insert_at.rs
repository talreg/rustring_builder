use rustring_builder::StringBuilder;

#[test]
fn test_insert_at(){
    let mut builder = StringBuilder::new();
    builder.append("hello world!");
    builder.insert_at(5, " cruel");
    assert_eq!("hello cruel world!", builder.to_string());

}

#[test]
fn test_insert_at_with_len(){
    let mut builder = StringBuilder::new();
    builder.append("hello world!");
    let sentence=" cruel, isn't it?";
    builder.insert_at_with_len(5, sentence, 6);
    assert_eq!("hello cruel world!", builder.to_string());
}