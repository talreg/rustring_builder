use string_builder::StringBuilder;

#[test]
fn test_create(){
    let sample=StringBuilder::new();
    assert!(sample.is_empty());
}

#[test]
fn test_to_string(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    assert_eq!("hello world",sample.to_string());
}