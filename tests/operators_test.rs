use rustring_builder::StringBuilder;

#[test]
fn test_concat_two_string_builders_into_a_third(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    let mut sample2=StringBuilder::new();
    sample2.append(" goodbye world");
    let sample3=sample+sample2;
    assert_eq!("hello world goodbye world",sample3.to_string());
}
