use rustring_builder::StringBuilder;

#[test]
fn test_types(){
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


#[test]
fn test_appending_different_types(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    sample.append(123);
    sample.append(123.456);
    assert_eq!("hello world123123.456",sample.to_string());
}