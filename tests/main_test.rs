use rustring_builder::StringBuilder;

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

#[test]
fn test_append_str(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    assert_eq!("hello world",sample.to_string());
}

#[test]
fn test_len(){
    let mut sample =StringBuilder::new();
    assert_eq!(0,sample.len());
    sample.append("123");
    assert_eq!(3,sample.len());
}

#[test]
fn test_n_elements(){
    let mut sample=StringBuilder::new();
    assert_eq!("",sample.to_string());
    sample.append(123);
    assert_eq!('2',sample.nth(1).unwrap());
    assert_eq!("123",sample.to_string());
}

#[test]
fn test_concatenation(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    sample.new_line();
    sample.append("goodbye world");
    assert_eq!("hello world\ngoodbye world",sample.to_string());
}

#[test]
fn test_bad_ntn_access(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    assert_eq!(None,sample.nth(11));
}


#[test]
fn test_add_another_string_builder(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    let mut sample2=StringBuilder::new();
    sample2.append(" goodbye world");
    sample+=sample2;
    assert_eq!("hello world goodbye world",sample.to_string());
}

#[test]
fn test_ntn(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    assert_eq!('o',sample.nth(4).unwrap());
}