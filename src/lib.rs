pub mod string_builder;


#[cfg(test)]
mod tests {
    use crate::string_builder::StringBuilder;
    #[test]
    fn it_works() {
        let mut builder=StringBuilder::new();
        builder.append(1);
        assert_eq!("1",builder.to_string())
    }
}
